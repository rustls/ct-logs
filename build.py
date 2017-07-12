 # -*- coding: utf-8 -*-
import subprocess
import sys
import json

HEADER = """//!
//! This library is automatically generated from Google's list of known CT
//! logs.  Don't edit it.
//!
//! The generation is done deterministically so you can verify it
//! yourself by inspecting and re-running the generation process.
//!

extern crate sct;
"""

LOG_LIST = 'https://www.gstatic.com/ct/log_list/log_list.json'
LOG_LIST_SIG = 'https://www.gstatic.com/ct/log_list/log_list.sig'

def fetch_and_check_sig():
    for cmd in (['curl', '-o', 'log_list.sig', LOG_LIST_SIG],
                ['curl', '-o', 'log_list.json', LOG_LIST],
                ['openssl', 'dgst', '-sha256', '-verify', 
                 'log_list_pubkey.pem', '-signature', 'log_list.sig', 'log_list.json']):
        subprocess.check_call(cmd)
    return json.load(open('log_list.json'))

def split_bundle(bundle):
    cert = ''
    for line in bundle.splitlines():
        if line.strip() != '':
            cert += line + '\n'
        if '-----END CERTIFICATE-----' in line:
            yield cert
            cert = ''

def calc_spki_hash(cert):
    """
    Use openssl to sha256 hash the public key in the certificate.
    """
    proc = subprocess.Popen(
            ['openssl', 'x509', '-noout', '-sha256', '-fingerprint'],
            stdin = subprocess.PIPE,
            stdout = subprocess.PIPE)
    stdout, _ = proc.communicate(cert)
    assert proc.returncode == 0
    assert stdout.startswith('SHA256 Fingerprint=')
    hash = stdout.replace('SHA256 Fingerprint=', '').replace(':', '')
    hash = hash.strip()
    assert len(hash) == 64
    return hash.lower()

def extract_header_spki_hash(cert):
    """
    Extract the sha256 hash of the public key in the header, for
    cross-checking.
    """
    line = [ll for ll in cert.splitlines() if ll.startswith('# SHA256 Fingerprint: ')][0]
    return line.replace('# SHA256 Fingerprint: ', '').replace(':', '').lower()

def unwrap_pem(cert):
    start = '-----BEGIN CERTIFICATE-----\n'
    end = '-----END CERTIFICATE-----\n'
    base64 = cert[cert.index(start)+len(start):cert.rindex(end)]
    return base64.decode('base64')

def extract(msg, name):
    lines = msg.splitlines()
    value = [ll for ll in lines if ll.startswith(name + ': ')][0]
    return value[len(name) + 2:].strip()

def convert_cert(cert_der):
    proc = subprocess.Popen(
            ['target/debug/process_cert'],
            stdin = subprocess.PIPE,
            stdout = subprocess.PIPE)
    stdout, _ = proc.communicate(cert_der)
    assert proc.returncode == 0
    return dict(
            subject = extract(stdout, 'Subject'),
            spki = extract(stdout, 'SPKI'),
            name_constraints = extract(stdout, 'Name-Constraints'))

def commentify(cert):
    lines = cert.splitlines()
    lines = [ll[2:] if ll.startswith('# ') else ll for ll in lines]
    return '/*\n   * ' + ('\n   * '.join(lines)) + '\n   */'

def convert_bytes(hex):
    bb = hex.decode('hex')
    return bb.encode('string_escape').replace('"', '\\"')

def print_root(cert, data):
    subject = convert_bytes(data['subject'])
    spki = convert_bytes(data['spki'])
    nc = data['name_constraints']
    nc = ('Some(b"%s")' % convert_bytes(nc)) if nc != 'None' else nc

    print """  %s
  webpki::TrustAnchor {
    subject: b"%s",
    spki: b"%s",
    name_constraints: %s
  },
""" % (commentify(cert), subject, spki, nc)

if __name__ == '__main__':
    if sys.platform == "win32":
        import os, msvcrt
        msvcrt.setmode(sys.stdout.fileno(), os.O_BINARY)

    json = fetch_and_check_sig()
    print json

    print HEADER
