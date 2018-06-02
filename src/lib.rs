//!
//! This library is automatically generated from Google's list of known CT
//! logs.  Don't edit it.
//!
//! The generation is done deterministically so you can verify it
//! yourself by inspecting and re-running the generation process.
//!

extern crate sct;

pub static LOGS: [&sct::Log; 27] = [
    /*
     * {
     *   "description": "Venafi Gen2 CT log",
     *   "dns_api_endpoint": "venafi2.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEjicnerZVCXTrbEuUhGW85BXx6lrYfA43zro/bAna5ymW00VQb94etBzSg4j/KS/Oqf/fNN51D8DMGA2ULvw3AQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     6
     *   ],
     *   "url": "ctlog-gen2.api.venafi.com/"
     * }
     */
    &sct::Log {
        description: "Venafi Gen2 CT log",
        url: "ctlog-gen2.api.venafi.com/",
        operated_by: "Venafi",
        key: b"\x04\x8e\'\'z\xb6U\tt\xeblK\x94\x84e\xbc\xe4\x15\xf1\xeaZ\xd8|\x0e7\xce\xba?l\t\xda\xe7)\x96\xd3EPo\xde\x1e\xb4\x1c\xd2\x83\x88\xff)/\xce\xa9\xff\xdf4\xdeu\x0f\xc0\xcc\x18\r\x94.\xfc7\x01",
        id: [ 0x03, 0x01, 0x9d, 0xf3, 0xfd, 0x85, 0xa6, 0x9a, 0x8e, 0xbd, 0x1f, 0xac, 0xc6, 0xda, 0x9b, 0xa7, 0x3e, 0x46, 0x97, 0x74, 0xfe, 0x77, 0xf5, 0x79, 0xfc, 0x5a, 0x08, 0xb8, 0x32, 0x8c, 0x1d, 0x6b ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Symantec 'Sirius' log",
     *   "dns_api_endpoint": "symantec-sirius.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEowJkhCK7JewN47zCyYl93UXQ7uYVhY/Z5xcbE4Dq7bKFN61qxdglnfr0tPNuFiglN+qjN2Syxwv9UeXBBfQOtQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "sirius.ws.symantec.com/"
     * }
     */
    &sct::Log {
        description: "Symantec 'Sirius' log",
        url: "sirius.ws.symantec.com/",
        operated_by: "DigiCert",
        key: b"\x04\xa3\x02d\x84\"\xbb%\xec\r\xe3\xbc\xc2\xc9\x89}\xddE\xd0\xee\xe6\x15\x85\x8f\xd9\xe7\x17\x1b\x13\x80\xea\xed\xb2\x857\xadj\xc5\xd8%\x9d\xfa\xf4\xb4\xf3n\x16(%7\xea\xa37d\xb2\xc7\x0b\xfdQ\xe5\xc1\x05\xf4\x0e\xb5",
        id: [ 0x15, 0x97, 0x04, 0x88, 0xd7, 0xb9, 0x97, 0xa0, 0x5b, 0xeb, 0x52, 0x51, 0x2a, 0xde, 0xe8, 0xd2, 0xe8, 0xb4, 0xa3, 0x16, 0x52, 0x64, 0x12, 0x1a, 0x9f, 0xab, 0xfb, 0xd5, 0xf8, 0x5a, 0xd9, 0x3f ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2022 Log",
     *   "dns_api_endpoint": "digicert-yeti2022.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEn/jYHd77W1G1+131td5mEbCdX/1v/KiYW5hPLcOROvv+xA8Nw2BDjB7y+RGyutD2vKXStp/5XIeiffzUfdYTJg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2022.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2022 Log",
        url: "yeti2022.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x9f\xf8\xd8\x1d\xde\xfb[Q\xb5\xfb]\xf5\xb5\xdef\x11\xb0\x9d_\xfdo\xfc\xa8\x98[\x98O-\xc3\x91:\xfb\xfe\xc4\x0f\r\xc3`C\x8c\x1e\xf2\xf9\x11\xb2\xba\xd0\xf6\xbc\xa5\xd2\xb6\x9f\xf9\\\x87\xa2}\xfc\xd4}\xd6\x13&",
        id: [ 0x22, 0x45, 0x45, 0x07, 0x59, 0x55, 0x24, 0x56, 0x96, 0x3f, 0xa1, 0x2f, 0xf1, 0xf7, 0x6d, 0x86, 0xe0, 0x23, 0x26, 0x63, 0xad, 0xc0, 0x4b, 0x7f, 0x5d, 0xc6, 0x83, 0x5c, 0x6e, 0xe2, 0x0f, 0x02 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Icarus' log",
     *   "dns_api_endpoint": "icarus.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAETtK8v7MICve56qTHHDhhBOuV4IlUaESxZryCfk9QbG9co/CqPvTsgPDbCpp6oFtyAHwlDhnvr7JijXRD9Cb2FA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/icarus/"
     * }
     */
    &sct::Log {
        description: "Google 'Icarus' log",
        url: "ct.googleapis.com/icarus/",
        operated_by: "Google",
        key: b"\x04N\xd2\xbc\xbf\xb3\x08\n\xf7\xb9\xea\xa4\xc7\x1c8a\x04\xeb\x95\xe0\x89ThD\xb1f\xbc\x82~OPlo\\\xa3\xf0\xaa>\xf4\xec\x80\xf0\xdb\n\x9az\xa0[r\x00|%\x0e\x19\xef\xaf\xb2b\x8dtC\xf4&\xf6\x14",
        id: [ 0x29, 0x3c, 0x51, 0x96, 0x54, 0xc8, 0x39, 0x65, 0xba, 0xaa, 0x50, 0xfc, 0x58, 0x07, 0xd4, 0xb7, 0x6f, 0xbf, 0x58, 0x7a, 0x29, 0x72, 0xdc, 0xa4, 0xc3, 0x0c, 0xf4, 0xe5, 0x45, 0x47, 0xf4, 0x78 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2021' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpon7ipsqehIeU1bmpog9TFo4Pk8+9oN8OYHl1Q2JGVXnkVFnuuvPgSo2Ep+6vLffNLcmEbxOucz03sFiematg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2021/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2021' Log",
        url: "ct.cloudflare.com/logs/nimbus2021/",
        operated_by: "Cloudflare",
        key: b"\x04\xc6\x9a\'\xee*l\xa9\xe8HyM[\x9a\x9a \xf51h\xe0\xf9<\xfb\xda\r\xf0\xe6\x07\x97T6$eW\x9eEE\x9e\xeb\xaf>\x04\xa8\xd8J~\xea\xf2\xdf|\xd2\xdc\x98F\xf1:\xe73\xd3{\x05\x89\xe9\x9a\xb6",
        id: [ 0x44, 0x94, 0x65, 0x2e, 0xb0, 0xee, 0xce, 0xaf, 0xc4, 0x40, 0x07, 0xd8, 0xa8, 0xfe, 0x28, 0xc0, 0xda, 0xe6, 0x82, 0xbe, 0xd8, 0xcb, 0x31, 0xb5, 0x3f, 0xd3, 0x33, 0x96, 0xb5, 0xb6, 0x81, 0xa8 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Comodo 'Sabre' CT log",
     *   "dns_api_endpoint": "comodo-sabre.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE8m/SiQ8/xfiHHqtls9m7FyOMBg4JVZY9CgiixXGz0akvKD6DEL8S0ERmFe9U4ZiA0M4kbT5nmuk3I85Sk4bagA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     9
     *   ],
     *   "url": "sabre.ct.comodo.com/"
     * }
     */
    &sct::Log {
        description: "Comodo 'Sabre' CT log",
        url: "sabre.ct.comodo.com/",
        operated_by: "Comodo CA Limited",
        key: b"\x04\xf2o\xd2\x89\x0f?\xc5\xf8\x87\x1e\xabe\xb3\xd9\xbb\x17#\x8c\x06\x0e\tU\x96=\n\x08\xa2\xc5q\xb3\xd1\xa9/(>\x83\x10\xbf\x12\xd0Df\x15\xefT\xe1\x98\x80\xd0\xce$m>g\x9a\xe97#\xceR\x93\x86\xda\x80",
        id: [ 0x55, 0x81, 0xd4, 0xc2, 0x16, 0x90, 0x36, 0x01, 0x4a, 0xea, 0x0b, 0x9b, 0x57, 0x3c, 0x53, 0xf0, 0xc0, 0xe4, 0x38, 0x78, 0x70, 0x25, 0x08, 0x17, 0x2f, 0xa3, 0xaa, 0x1d, 0x07, 0x13, 0xd3, 0x0c ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Log Server",
     *   "dns_api_endpoint": "digicert.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEAkbFvhu7gkAW6MHSrBlpE1n4+HCFRkC5OLAjgqhkTH+/uzSfSl8ois8ZxAD2NgaTZe1M9akhYlrYkes4JECs6A==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "ct1.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server",
        url: "ct1.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x02F\xc5\xbe\x1b\xbb\x82@\x16\xe8\xc1\xd2\xac\x19i\x13Y\xf8\xf8p\x85F@\xb98\xb0#\x82\xa8dL\x7f\xbf\xbb4\x9fJ_(\x8a\xcf\x19\xc4\x00\xf66\x06\x93e\xedL\xf5\xa9!bZ\xd8\x91\xeb8$@\xac\xe8",
        id: [ 0x56, 0x14, 0x06, 0x9a, 0x2f, 0xd7, 0xc2, 0xec, 0xd3, 0xf5, 0xe1, 0xbd, 0x44, 0xb2, 0x3e, 0xc7, 0x46, 0x76, 0xb9, 0xbc, 0x99, 0x11, 0x5c, 0xc0, 0xef, 0x94, 0x98, 0x55, 0xd6, 0x89, 0xd0, 0xdd ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2021 Log",
     *   "dns_api_endpoint": "digicert-yeti2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6J4EbcpIAl1+AkSRsbhoY5oRTj3VoFfaf1DlQkfi7Rbe/HcjfVtrwN8jaC+tQDGjF+dqvKhWJAQ6Q6ev6q9Mew==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2021.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2021 Log",
        url: "yeti2021.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xe8\x9e\x04m\xcaH\x02]~\x02D\x91\xb1\xb8hc\x9a\x11N=\xd5\xa0W\xda\x7fP\xe5BG\xe2\xed\x16\xde\xfcw#}[k\xc0\xdf#h/\xad@1\xa3\x17\xe7j\xbc\xa8V$\x04:C\xa7\xaf\xea\xafL{",
        id: [ 0x5c, 0xdc, 0x43, 0x92, 0xfe, 0xe6, 0xab, 0x45, 0x44, 0xb1, 0x5e, 0x9a, 0xd4, 0x56, 0xe6, 0x10, 0x37, 0xfb, 0xd5, 0xfa, 0x47, 0xdc, 0xa1, 0x73, 0x94, 0xb2, 0x5e, 0xe6, 0xf6, 0xc7, 0x0e, 0xca ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2020' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE01EAhx4o0zPQrXTcYjgCt4MVFsT0Pwjzb1RwrM0lhWDlxAYPP6/gyMCXNkOn/7KFsjL7rwk78tHMpY8rXn8AYg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2020/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2020' Log",
        url: "ct.cloudflare.com/logs/nimbus2020/",
        operated_by: "Cloudflare",
        key: b"\x04\xd3Q\x00\x87\x1e(\xd33\xd0\xadt\xdcb8\x02\xb7\x83\x15\x16\xc4\xf4?\x08\xf3oTp\xac\xcd%\x85`\xe5\xc4\x06\x0f?\xaf\xe0\xc8\xc0\x976C\xa7\xff\xb2\x85\xb22\xfb\xaf\t;\xf2\xd1\xcc\xa5\x8f+^\x7f\x00b",
        id: [ 0x5e, 0xa7, 0x73, 0xf9, 0xdf, 0x56, 0xc0, 0xe7, 0xb5, 0x36, 0x48, 0x7d, 0xd0, 0x49, 0xe0, 0x32, 0x7a, 0x91, 0x9a, 0x0c, 0x84, 0xa1, 0x12, 0x12, 0x84, 0x18, 0x75, 0x96, 0x81, 0x71, 0x45, 0x58 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2019' log",
     *   "dns_api_endpoint": "argon2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEI3MQm+HzXvaYa2mVlhB4zknbtAT8cSxakmBoJcBKGqGwYS0bhxSpuvABM1kdBTDpQhXnVdcq+LSiukXJRpGHVg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2019/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2019' log",
        url: "ct.googleapis.com/logs/argon2019/",
        operated_by: "Google",
        key: b"\x04#s\x10\x9b\xe1\xf3^\xf6\x98ki\x95\x96\x10x\xceI\xdb\xb4\x04\xfcq,Z\x92`h%\xc0J\x1a\xa1\xb0a-\x1b\x87\x14\xa9\xba\xf0\x013Y\x1d\x050\xe9B\x15\xe7U\xd7*\xf8\xb4\xa2\xbaE\xc9F\x91\x87V",
        id: [ 0x63, 0xf2, 0xdb, 0xcd, 0xe8, 0x3b, 0xcc, 0x2c, 0xcf, 0x0b, 0x72, 0x84, 0x27, 0x57, 0x6b, 0x33, 0xa4, 0x8d, 0x61, 0x77, 0x8f, 0xbd, 0x75, 0xa6, 0x38, 0xb1, 0xc7, 0x68, 0x54, 0x4b, 0xd8, 0x8d ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Aviator' log",
     *   "dns_api_endpoint": "aviator.ct.googleapis.com",
     *   "final_sth": {
     *     "sha256_root_hash": "LcGcZRsm+LGYmrlyC5LXhV1T6OD8iH5dNlb0sEJl9bA=",
     *     "timestamp": 1480512258330,
     *     "tree_head_signature": "BAMASDBGAiEA/M0Nvt77aNe+9eYbKsv6rRpTzFTKa5CGqb56ea4hnt8CIQCJDE7pL6xgAewMd5i3G1lrBWgFooT2kd3+zliEz5Rw8w==",
     *     "tree_size": 46466472
     *   },
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE1/TMabLkDpCjiupacAlP7xNi0I1JYP8bQFAHDG1xhtolSY1l4QgNRzRrvSe8liE+NPWHdjGxfx3JhTsN9x8/6Q==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/aviator/"
     * }
     */
    &sct::Log {
        description: "Google 'Aviator' log",
        url: "ct.googleapis.com/aviator/",
        operated_by: "Google",
        key: b"\x04\xd7\xf4\xcci\xb2\xe4\x0e\x90\xa3\x8a\xeaZp\tO\xef\x13b\xd0\x8dI`\xff\x1b@P\x07\x0cmq\x86\xda%I\x8de\xe1\x08\rG4k\xbd\'\xbc\x96!>4\xf5\x87v1\xb1\x7f\x1d\xc9\x85;\r\xf7\x1f?\xe9",
        id: [ 0x68, 0xf6, 0x98, 0xf8, 0x1f, 0x64, 0x82, 0xbe, 0x3a, 0x8c, 0xee, 0xb9, 0x28, 0x1d, 0x4c, 0xfc, 0x71, 0x51, 0x5d, 0x67, 0x93, 0xd4, 0x44, 0xd1, 0x0a, 0x67, 0xac, 0xbb, 0x4f, 0x4f, 0xfb, 0xc4 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Comodo 'Mammoth' CT log",
     *   "dns_api_endpoint": "comodo-mammoth.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE7+R9dC4VFbbpuyOL+yy14ceAmEf7QGlo/EmtYU6DRzwat43f/3swtLr/L8ugFOOt1YU/RFmMjGCL17ixv66MZw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     9
     *   ],
     *   "url": "mammoth.ct.comodo.com/"
     * }
     */
    &sct::Log {
        description: "Comodo 'Mammoth' CT log",
        url: "mammoth.ct.comodo.com/",
        operated_by: "Comodo CA Limited",
        key: b"\x04\xef\xe4}t.\x15\x15\xb6\xe9\xbb#\x8b\xfb,\xb5\xe1\xc7\x80\x98G\xfb@ih\xfcI\xadaN\x83G<\x1a\xb7\x8d\xdf\xff{0\xb4\xba\xff/\xcb\xa0\x14\xe3\xad\xd5\x85?DY\x8c\x8c`\x8b\xd7\xb8\xb1\xbf\xae\x8cg",
        id: [ 0x6f, 0x53, 0x76, 0xac, 0x31, 0xf0, 0x31, 0x19, 0xd8, 0x99, 0x00, 0xa4, 0x51, 0x15, 0xff, 0x77, 0x15, 0x1c, 0x11, 0xd9, 0x02, 0xc1, 0x00, 0x29, 0x06, 0x8d, 0xb2, 0x08, 0x9a, 0x37, 0xd9, 0x13 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2019' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEkZHz1v5r8a9LmXSMegYZAg4UW+Ug56GtNfJTDNFZuubEJYgWf4FcC5D+ZkYwttXTDSo4OkanG9b3AI4swIQ28g==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2019/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2019' Log",
        url: "ct.cloudflare.com/logs/nimbus2019/",
        operated_by: "Cloudflare",
        key: b"\x04\x91\x91\xf3\xd6\xfek\xf1\xafK\x99t\x8cz\x06\x19\x02\x0e\x14[\xe5 \xe7\xa1\xad5\xf2S\x0c\xd1Y\xba\xe6\xc4%\x88\x16\x7f\x81\\\x0b\x90\xfefF0\xb6\xd5\xd3\r*8:F\xa7\x1b\xd6\xf7\x00\x8e,\xc0\x846\xf2",
        id: [ 0x74, 0x7e, 0xda, 0x83, 0x31, 0xad, 0x33, 0x10, 0x91, 0x21, 0x9c, 0xce, 0x25, 0x4f, 0x42, 0x70, 0xc2, 0xbf, 0xfd, 0x5e, 0x42, 0x20, 0x08, 0xc6, 0x37, 0x35, 0x79, 0xe6, 0x10, 0x7b, 0xcc, 0x56 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Log Server 2",
     *   "dns_api_endpoint": "digicert2.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEzF05L2a4TH/BLgOhNKPoioYCrkoRxvcmajeb8Dj4XQmNY+gxa4Zmz3mzJTwe33i0qMVp+rfwgnliQ/bM/oFmhA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "ct2.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server 2",
        url: "ct2.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\xcc]9/f\xb8L\x7f\xc1.\x03\xa14\xa3\xe8\x8a\x86\x02\xaeJ\x11\xc6\xf7&j7\x9b\xf08\xf8]\t\x8dc\xe81k\x86f\xcfy\xb3%<\x1e\xdfx\xb4\xa8\xc5i\xfa\xb7\xf0\x82ybC\xf6\xcc\xfe\x81f\x84",
        id: [ 0x87, 0x75, 0xbf, 0xe7, 0x59, 0x7c, 0xf8, 0x8c, 0x43, 0x99, 0x5f, 0xbd, 0xf3, 0x6e, 0xff, 0x56, 0x8d, 0x47, 0x56, 0x36, 0xff, 0x4a, 0xb5, 0x60, 0xc1, 0xb4, 0xea, 0xff, 0x5e, 0xa0, 0x83, 0x0f ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2018' log",
     *   "dns_api_endpoint": "argon2018.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE0gBVBa3VR7QZu82V+ynXWD14JM3ORp37MtRxTmACJV5ZPtfUA7htQ2hofuigZQs+bnFZkje+qejxoyvk2Q1VaA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2018/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2018' log",
        url: "ct.googleapis.com/logs/argon2018/",
        operated_by: "Google",
        key: b"\x04\xd2\x00U\x05\xad\xd5G\xb4\x19\xbb\xcd\x95\xfb)\xd7X=x$\xcd\xceF\x9d\xfb2\xd4qN`\x02%^Y>\xd7\xd4\x03\xb8mChh~\xe8\xa0e\x0b>nqY\x927\xbe\xa9\xe8\xf1\xa3+\xe4\xd9\rUh",
        id: [ 0xa4, 0x50, 0x12, 0x69, 0x05, 0x5a, 0x15, 0x54, 0x5e, 0x62, 0x11, 0xab, 0x37, 0xbc, 0x10, 0x3f, 0x62, 0xae, 0x55, 0x76, 0xa4, 0x5e, 0x4b, 0x17, 0x14, 0x45, 0x3e, 0x1b, 0x22, 0x10, 0x6a, 0x25 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Pilot' log",
     *   "dns_api_endpoint": "pilot.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEfahLEimAoz2t01p3uMziiLOl/fHTDM0YDOhBRuiBARsV4UvxG2LdNgoIGLrtCzWE0J5APC2em4JlvR8EEEFMoA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/pilot/"
     * }
     */
    &sct::Log {
        description: "Google 'Pilot' log",
        url: "ct.googleapis.com/pilot/",
        operated_by: "Google",
        key: b"\x04}\xa8K\x12)\x80\xa3=\xad\xd3Zw\xb8\xcc\xe2\x88\xb3\xa5\xfd\xf1\xd3\x0c\xcd\x18\x0c\xe8AF\xe8\x81\x01\x1b\x15\xe1K\xf1\x1bb\xdd6\n\x08\x18\xba\xed\x0b5\x84\xd0\x9e@<-\x9e\x9b\x82e\xbd\x1f\x04\x10AL\xa0",
        id: [ 0xa4, 0xb9, 0x09, 0x90, 0xb4, 0x18, 0x58, 0x14, 0x87, 0xbb, 0x13, 0xa2, 0xcc, 0x67, 0x70, 0x0a, 0x3c, 0x35, 0x98, 0x04, 0xf9, 0x1b, 0xdf, 0xb8, 0xe3, 0x77, 0xcd, 0x0e, 0xc8, 0x0d, 0xdc, 0x10 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "CNNIC CT log",
     *   "dns_api_endpoint": "cnnic.ct.googleapis.com",
     *   "key": "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAv7UIYZopMgTTJWPp2IXhhuAf1l6a9zM7gBvntj5fLaFm9pVKhKYhVnno94XuXeN8EsDgiSIJIj66FpUGvai5samyetZhLocRuXhAiXXbDNyQ4KR51tVebtEq2zT0mT9liTtGwiksFQccyUsaVPhsHq9gJ2IKZdWauVA2Fm5x9h8B9xKn/L/2IaMpkIYtd967TNTP/dLPgixN1PLCLaypvurDGSVDsuWabA3FHKWL9z8wr7kBkbdpEhLlg2H+NAC+9nGKx+tQkuhZ/hWR65aX+CNUPy2OB9/u2rNPyDydb988LENXoUcMkQT0dU3aiYGkFAY0uZjD2vH97TM20xYtNQIDAQAB",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     7
     *   ],
     *   "url": "ctserver.cnnic.cn/"
     * }
     */
    &sct::Log {
        description: "CNNIC CT log",
        url: "ctserver.cnnic.cn/",
        operated_by: "CNNIC",
        key: b"0\x82\x01\n\x02\x82\x01\x01\x00\xbf\xb5\x08a\x9a)2\x04\xd3%c\xe9\xd8\x85\xe1\x86\xe0\x1f\xd6^\x9a\xf73;\x80\x1b\xe7\xb6>_-\xa1f\xf6\x95J\x84\xa6!Vy\xe8\xf7\x85\xee]\xe3|\x12\xc0\xe0\x89\"\t\">\xba\x16\x95\x06\xbd\xa8\xb9\xb1\xa9\xb2z\xd6a.\x87\x11\xb9x@\x89u\xdb\x0c\xdc\x90\xe0\xa4y\xd6\xd5^n\xd1*\xdb4\xf4\x99?e\x89;F\xc2),\x15\x07\x1c\xc9K\x1aT\xf8l\x1e\xaf`\'b\ne\xd5\x9a\xb9P6\x16nq\xf6\x1f\x01\xf7\x12\xa7\xfc\xbf\xf6!\xa3)\x90\x86-w\xde\xbbL\xd4\xcf\xfd\xd2\xcf\x82,M\xd4\xf2\xc2-\xac\xa9\xbe\xea\xc3\x19%C\xb2\xe5\x9al\r\xc5\x1c\xa5\x8b\xf7?0\xaf\xb9\x01\x91\xb7i\x12\x12\xe5\x83a\xfe4\x00\xbe\xf6q\x8a\xc7\xebP\x92\xe8Y\xfe\x15\x91\xeb\x96\x97\xf8#T?-\x8e\x07\xdf\xee\xda\xb3O\xc8<\x9do\xdf<,CW\xa1G\x0c\x91\x04\xf4uM\xda\x89\x81\xa4\x14\x064\xb9\x98\xc3\xda\xf1\xfd\xed36\xd3\x16-5\x02\x03\x01\x00\x01",
        id: [ 0xa5, 0x77, 0xac, 0x9c, 0xed, 0x75, 0x48, 0xdd, 0x8f, 0x02, 0x5b, 0x67, 0xa2, 0x41, 0x08, 0x9d, 0xf8, 0x6e, 0x0f, 0x47, 0x6e, 0xc2, 0x03, 0xc2, 0xec, 0xbe, 0xdb, 0x18, 0x5f, 0x28, 0x26, 0x38 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2020' log",
     *   "dns_api_endpoint": "argon2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6Tx2p1yKY4015NyIYvdrk36es0uAc1zA4PQ+TGRY+3ZjUTIYY9Wyu+3q/147JG4vNVKLtDWarZwVqGkg6lAYzA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2020/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2020' log",
        url: "ct.googleapis.com/logs/argon2020/",
        operated_by: "Google",
        key: b"\x04\xe9<v\xa7\\\x8ac\x8d5\xe4\xdc\x88b\xf7k\x93~\x9e\xb3K\x80s\\\xc0\xe0\xf4>LdX\xfbvcQ2\x18c\xd5\xb2\xbb\xed\xea\xff^;$n/5R\x8b\xb45\x9a\xad\x9c\x15\xa8i \xeaP\x18\xcc",
        id: [ 0xb2, 0x1e, 0x05, 0xcc, 0x8b, 0xa2, 0xcd, 0x8a, 0x20, 0x4e, 0x87, 0x66, 0xf9, 0x2b, 0xb9, 0x8a, 0x25, 0x20, 0x67, 0x6b, 0xda, 0xfa, 0x70, 0xe7, 0xb2, 0x49, 0x53, 0x2d, 0xef, 0x8b, 0x90, 0x5e ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Skydiver' log",
     *   "dns_api_endpoint": "skydiver.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEEmyGDvYXsRJsNyXSrYc9DjHsIa2xzb4UR7ZxVoV6mrc9iZB7xjI6+NrOiwH+P/xxkRmOFG6Jel20q37hTh58rA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/skydiver/"
     * }
     */
    &sct::Log {
        description: "Google 'Skydiver' log",
        url: "ct.googleapis.com/skydiver/",
        operated_by: "Google",
        key: b"\x04\x12l\x86\x0e\xf6\x17\xb1\x12l7%\xd2\xad\x87=\x0e1\xec!\xad\xb1\xcd\xbe\x14G\xb6qV\x85z\x9a\xb7=\x89\x90{\xc62:\xf8\xda\xce\x8b\x01\xfe?\xfcq\x91\x19\x8e\x14n\x89z]\xb4\xab~\xe1N\x1e|\xac",
        id: [ 0xbb, 0xd9, 0xdf, 0xbc, 0x1f, 0x8a, 0x71, 0xb5, 0x93, 0x94, 0x23, 0x97, 0xaa, 0x92, 0x7b, 0x47, 0x38, 0x57, 0x95, 0x0a, 0xab, 0x52, 0xe8, 0x1a, 0x90, 0x96, 0x64, 0x36, 0x8e, 0x1e, 0xd1, 0x85 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Symantec 'Vega' log",
     *   "dns_api_endpoint": "symantec-vega.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6pWeAv/u8TNtS4e8zf0ZF2L/lNPQWQc/Ai0ckP7IRzA78d0NuBEMXR2G3avTK0Zm+25ltzv9WWis36b4ztIYTQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "vega.ws.symantec.com/"
     * }
     */
    &sct::Log {
        description: "Symantec 'Vega' log",
        url: "vega.ws.symantec.com/",
        operated_by: "DigiCert",
        key: b"\x04\xea\x95\x9e\x02\xff\xee\xf13mK\x87\xbc\xcd\xfd\x19\x17b\xff\x94\xd3\xd0Y\x07?\x02-\x1c\x90\xfe\xc8G0;\xf1\xdd\r\xb8\x11\x0c]\x1d\x86\xdd\xab\xd3+Ff\xfbne\xb7;\xfdYh\xac\xdf\xa6\xf8\xce\xd2\x18M",
        id: [ 0xbc, 0x78, 0xe1, 0xdf, 0xc5, 0xf6, 0x3c, 0x68, 0x46, 0x49, 0x33, 0x4d, 0xa1, 0x0f, 0xa1, 0x5f, 0x09, 0x79, 0x69, 0x20, 0x09, 0xc0, 0x81, 0xb4, 0xf3, 0xf6, 0x91, 0x7f, 0x3e, 0xd9, 0xb8, 0xa5 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2018 Log",
     *   "dns_api_endpoint": "digicert-yeti2018.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAESYlKFDLLFmA9JScaiaNnqlU8oWDytxIYMfswHy9Esg0aiX+WnP/yj4O0ViEHtLwbmOQeSWBGkIu9YK9CLeer+g==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2018.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2018 Log",
        url: "yeti2018.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04I\x89J\x142\xcb\x16`=%\'\x1a\x89\xa3g\xaaU<\xa1`\xf2\xb7\x12\x181\xfb0\x1f/D\xb2\r\x1a\x89\x7f\x96\x9c\xff\xf2\x8f\x83\xb4V!\x07\xb4\xbc\x1b\x98\xe4\x1eI`F\x90\x8b\xbd`\xafB-\xe7\xab\xfa",
        id: [ 0xc1, 0x16, 0x4a, 0xe0, 0xa7, 0x72, 0xd2, 0xd4, 0x39, 0x2d, 0xc8, 0x0a, 0xc1, 0x07, 0x70, 0xd4, 0xf0, 0xc4, 0x9b, 0xde, 0x99, 0x1a, 0x48, 0x40, 0xc1, 0xfa, 0x07, 0x51, 0x64, 0xf6, 0x33, 0x60 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Cloudflare 'Nimbus2018' Log",
     *   "dns_api_endpoint": "cloudflare-nimbus2018.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEAsVpWvrH3Ke0VRaMg9ZQoQjb5g/xh1z3DDa6IuxY5DyPsk6brlvrUNXZzoIg0DcvFiAn2kd6xmu4Obk5XA/nRg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct.cloudflare.com/logs/nimbus2018/"
     * }
     */
    &sct::Log {
        description: "Cloudflare 'Nimbus2018' Log",
        url: "ct.cloudflare.com/logs/nimbus2018/",
        operated_by: "Cloudflare",
        key: b"\x04\x02\xc5iZ\xfa\xc7\xdc\xa7\xb4U\x16\x8c\x83\xd6P\xa1\x08\xdb\xe6\x0f\xf1\x87\\\xf7\x0c6\xba\"\xecX\xe4<\x8f\xb2N\x9b\xae[\xebP\xd5\xd9\xce\x82 \xd07/\x16 \'\xdaGz\xc6k\xb89\xb99\\\x0f\xe7F",
        id: [ 0xdb, 0x74, 0xaf, 0xee, 0xcb, 0x29, 0xec, 0xb1, 0xfe, 0xca, 0x3e, 0x71, 0x6d, 0x2c, 0xe5, 0xb9, 0xaa, 0xbb, 0x36, 0xf7, 0x84, 0x71, 0x83, 0xc7, 0x5d, 0x9d, 0x4f, 0x37, 0xb6, 0x1f, 0xbf, 0x64 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Symantec log",
     *   "dns_api_endpoint": "symantec.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEluqsHEYMG1XcDfy1lCdGV0JwOmkY4r87xNuroPS2bMBTP01CEDPwWJePa75y9CrsHEKqAy8afig1dpkIPSEUhg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "ct.ws.symantec.com/"
     * }
     */
    &sct::Log {
        description: "Symantec log",
        url: "ct.ws.symantec.com/",
        operated_by: "DigiCert",
        key: b"\x04\x96\xea\xac\x1cF\x0c\x1bU\xdc\r\xfc\xb5\x94\'FWBp:i\x18\xe2\xbf;\xc4\xdb\xab\xa0\xf4\xb6l\xc0S?MB\x103\xf0X\x97\x8fk\xber\xf4*\xec\x1cB\xaa\x03/\x1a~(5v\x99\x08=!\x14\x86",
        id: [ 0xdd, 0xeb, 0x1d, 0x2b, 0x7a, 0x0d, 0x4f, 0xa6, 0x20, 0x8b, 0x81, 0xad, 0x81, 0x68, 0x70, 0x7e, 0x2e, 0x8e, 0x9d, 0x01, 0xd5, 0x5c, 0x88, 0x8d, 0x3d, 0x11, 0xc4, 0xcd, 0xb6, 0xec, 0xbe, 0xcc ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2019 Log",
     *   "dns_api_endpoint": "digicert-yeti2019.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEkZd/ow8X+FSVWAVSf8xzkFohcPph/x6pS1JHh7g1wnCZ5y/8Hk6jzJxs6t3YMAWz2CPd4VkCdxwKexGhcFxD9A==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2019.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2019 Log",
        url: "yeti2019.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04\x91\x97\x7f\xa3\x0f\x17\xf8T\x95X\x05R\x7f\xccs\x90Z!p\xfaa\xff\x1e\xa9KRG\x87\xb85\xc2p\x99\xe7/\xfc\x1eN\xa3\xcc\x9cl\xea\xdd\xd80\x05\xb3\xd8#\xdd\xe1Y\x02w\x1c\n{\x11\xa1p\\C\xf4",
        id: [ 0xe2, 0x69, 0x4b, 0xae, 0x26, 0xe8, 0xe9, 0x40, 0x09, 0xe8, 0x86, 0x1b, 0xb6, 0x3b, 0x83, 0xd4, 0x3e, 0xe7, 0xfe, 0x74, 0x88, 0xfb, 0xa4, 0x8f, 0x28, 0x93, 0x01, 0x9d, 0xdd, 0xf1, 0xdb, 0xfe ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Rocketeer' log",
     *   "dns_api_endpoint": "rocketeer.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEIFsYyDzBi7MxCAC/oJBXK7dHjG+1aLCOkHjpoHPqTyghLpzA9BYbqvnV16mAw04vUjyYASVGJCUoI3ctBcJAeg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/rocketeer/"
     * }
     */
    &sct::Log {
        description: "Google 'Rocketeer' log",
        url: "ct.googleapis.com/rocketeer/",
        operated_by: "Google",
        key: b"\x04 [\x18\xc8<\xc1\x8b\xb31\x08\x00\xbf\xa0\x90W+\xb7G\x8co\xb5h\xb0\x8e\x90x\xe9\xa0s\xeaO(!.\x9c\xc0\xf4\x16\x1b\xaa\xf9\xd5\xd7\xa9\x80\xc3N/R<\x98\x01%F$%(#w-\x05\xc2@z",
        id: [ 0xee, 0x4b, 0xbd, 0xb7, 0x75, 0xce, 0x60, 0xba, 0xe1, 0x42, 0x69, 0x1f, 0xab, 0xe1, 0x9e, 0x66, 0xa3, 0x0f, 0x7e, 0x5f, 0xb0, 0x72, 0xd8, 0x83, 0x00, 0xc4, 0x7b, 0x89, 0x7a, 0xa8, 0xfd, 0xcb ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Yeti2020 Log",
     *   "dns_api_endpoint": "digicert-yeti2020.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEURAG+Zo0ac3n37ifZKUhBFEV6jfcCzGIRz3tsq8Ca9BP/5XUHy6ZiqsPaAEbVM0uI3Tm9U24RVBHR9JxDElPmg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     2
     *   ],
     *   "url": "yeti2020.ct.digicert.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Yeti2020 Log",
        url: "yeti2020.ct.digicert.com/log/",
        operated_by: "DigiCert",
        key: b"\x04Q\x10\x06\xf9\x9a4i\xcd\xe7\xdf\xb8\x9fd\xa5!\x04Q\x15\xea7\xdc\x0b1\x88G=\xed\xb2\xaf\x02k\xd0O\xff\x95\xd4\x1f.\x99\x8a\xab\x0fh\x01\x1bT\xcd.#t\xe6\xf5M\xb8EPGG\xd2q\x0cIO\x9a",
        id: [ 0xf0, 0x95, 0xa4, 0x59, 0xf2, 0x00, 0xd1, 0x82, 0x40, 0x10, 0x2d, 0x2f, 0x93, 0x88, 0x8e, 0xad, 0x4b, 0xfe, 0x1d, 0x47, 0xe3, 0x99, 0xe1, 0xd0, 0x34, 0xa6, 0xb0, 0xa8, 0xaa, 0x8e, 0xb2, 0x73 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Google 'Argon2021' log",
     *   "dns_api_endpoint": "argon2021.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAETeBmZOrzZKo4xYktx9gI2chEce3cw/tbr5xkoQlmhB18aKfsxD+MnILgGNl0FOm0eYGilFVi85wLRIOhK8lxKw==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     0
     *   ],
     *   "url": "ct.googleapis.com/logs/argon2021/"
     * }
     */
    &sct::Log {
        description: "Google 'Argon2021' log",
        url: "ct.googleapis.com/logs/argon2021/",
        operated_by: "Google",
        key: b"\x04M\xe0fd\xea\xf3d\xaa8\xc5\x89-\xc7\xd8\x08\xd9\xc8Dq\xed\xdc\xc3\xfb[\xaf\x9cd\xa1\tf\x84\x1d|h\xa7\xec\xc4?\x8c\x9c\x82\xe0\x18\xd9t\x14\xe9\xb4y\x81\xa2\x94Ub\xf3\x9c\x0bD\x83\xa1+\xc9q+",
        id: [ 0xf6, 0x5c, 0x94, 0x2f, 0xd1, 0x77, 0x30, 0x22, 0x14, 0x54, 0x18, 0x08, 0x30, 0x94, 0x56, 0x8e, 0xe3, 0x4d, 0x13, 0x19, 0x33, 0xbf, 0xdf, 0x0c, 0x2f, 0x20, 0x0b, 0xcc, 0x4e, 0xf1, 0x64, 0xe3 ],
        max_merge_delay: 86400,
    },

];
