//!
//! This library is automatically generated from Google's list of known CT
//! logs.  Don't edit it.
//!
//! The generation is done deterministically so you can verify it
//! yourself by inspecting and re-running the generation process.
//!

extern crate sct;

pub static LOGS: [&sct::Log; 19] = [
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\x8e\'\'z\xb6U\tt\xeblK\x94\x84e\xbc\xe4\x15\xf1\xeaZ\xd8|\x0e7\xce\xba?l\t\xda\xe7)\x96\xd3EPo\xde\x1e\xb4\x1c\xd2\x83\x88\xff)/\xce\xa9\xff\xdf4\xdeu\x0f\xc0\xcc\x18\r\x94.\xfc7\x01",
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
        operated_by: "Symantec",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xa3\x02d\x84\"\xbb%\xec\r\xe3\xbc\xc2\xc9\x89}\xddE\xd0\xee\xe6\x15\x85\x8f\xd9\xe7\x17\x1b\x13\x80\xea\xed\xb2\x857\xadj\xc5\xd8%\x9d\xfa\xf4\xb4\xf3n\x16(%7\xea\xa37d\xb2\xc7\x0b\xfdQ\xe5\xc1\x05\xf4\x0e\xb5",
        id: [ 0x15, 0x97, 0x04, 0x88, 0xd7, 0xb9, 0x97, 0xa0, 0x5b, 0xeb, 0x52, 0x51, 0x2a, 0xde, 0xe8, 0xd2, 0xe8, 0xb4, 0xa3, 0x16, 0x52, 0x64, 0x12, 0x1a, 0x9f, 0xab, 0xfb, 0xd5, 0xf8, 0x5a, 0xd9, 0x3f ],
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04N\xd2\xbc\xbf\xb3\x08\n\xf7\xb9\xea\xa4\xc7\x1c8a\x04\xeb\x95\xe0\x89ThD\xb1f\xbc\x82~OPlo\\\xa3\xf0\xaa>\xf4\xec\x80\xf0\xdb\n\x9az\xa0[r\x00|%\x0e\x19\xef\xaf\xb2b\x8dtC\xf4&\xf6\x14",
        id: [ 0x29, 0x3c, 0x51, 0x96, 0x54, 0xc8, 0x39, 0x65, 0xba, 0xaa, 0x50, 0xfc, 0x58, 0x07, 0xd4, 0xb7, 0x6f, 0xbf, 0x58, 0x7a, 0x29, 0x72, 0xdc, 0xa4, 0xc3, 0x0c, 0xf4, 0xe5, 0x45, 0x47, 0xf4, 0x78 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "StartCom log",
     *   "dns_api_endpoint": "startcom1.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAESPNZ8/YFGNPbsu1Gfs/IEbVXsajWTOaft0oaFIZDqUiwy1o/PErK38SCFFWa+PeOQFXc9NKv6nV0+05/YIYuUQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     8
     *   ],
     *   "url": "ct.startssl.com/"
     * }
     */
    &sct::Log {
        description: "StartCom log",
        url: "ct.startssl.com/",
        operated_by: "StartSSL",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04H\xf3Y\xf3\xf6\x05\x18\xd3\xdb\xb2\xedF~\xcf\xc8\x11\xb5W\xb1\xa8\xd6L\xe6\x9f\xb7J\x1a\x14\x86C\xa9H\xb0\xcbZ?<J\xca\xdf\xc4\x82\x14U\x9a\xf8\xf7\x8e@U\xdc\xf4\xd2\xaf\xeaut\xfbN\x7f`\x86.Q",
        id: [ 0x34, 0xbb, 0x6a, 0xd6, 0xc3, 0xdf, 0x9c, 0x03, 0xee, 0xa8, 0xa4, 0x99, 0xff, 0x78, 0x91, 0x48, 0x6c, 0x9d, 0x5e, 0x5c, 0xac, 0x92, 0xd0, 0x1f, 0x7b, 0xfd, 0x1b, 0xce, 0x19, 0xdb, 0x48, 0xef ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "WoSign log",
     *   "dns_api_endpoint": "wosign1.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEzBGIey1my66PTTBmJxklIpMhRrQvAdPG+SvVyLpzmwai8IoCnNBrRhgwhbrpJIsO0VtwKAx+8TpFf1rzgkJgMQ==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     5
     *   ],
     *   "url": "ctlog.wosign.com/"
     * }
     */
    &sct::Log {
        description: "WoSign log",
        url: "ctlog.wosign.com/",
        operated_by: "Wosign",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xcc\x11\x88{-f\xcb\xae\x8fM0f\'\x19%\"\x93!F\xb4/\x01\xd3\xc6\xf9+\xd5\xc8\xbas\x9b\x06\xa2\xf0\x8a\x02\x9c\xd0kF\x180\x85\xba\xe9$\x8b\x0e\xd1[p(\x0c~\xf1:E\x7fZ\xf3\x82B`1",
        id: [ 0x41, 0xb2, 0xdc, 0x2e, 0x89, 0xe6, 0x3c, 0xe4, 0xaf, 0x1b, 0xa7, 0xbb, 0x29, 0xbf, 0x68, 0xc6, 0xde, 0xe6, 0xf9, 0xf1, 0xcc, 0x04, 0x7e, 0x30, 0xdf, 0xfa, 0xe3, 0xb3, 0xba, 0x25, 0x92, 0x63 ],
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
        operated_by: "Comodo",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xf2o\xd2\x89\x0f?\xc5\xf8\x87\x1e\xabe\xb3\xd9\xbb\x17#\x8c\x06\x0e\tU\x96=\n\x08\xa2\xc5q\xb3\xd1\xa9/(>\x83\x10\xbf\x12\xd0Df\x15\xefT\xe1\x98\x80\xd0\xce$m>g\x9a\xe97#\xceR\x93\x86\xda\x80",
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
     *     1
     *   ],
     *   "url": "ct1.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server",
        url: "ct1.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\x02F\xc5\xbe\x1b\xbb\x82@\x16\xe8\xc1\xd2\xac\x19i\x13Y\xf8\xf8p\x85F@\xb98\xb0#\x82\xa8dL\x7f\xbf\xbb4\x9fJ_(\x8a\xcf\x19\xc4\x00\xf66\x06\x93e\xedL\xf5\xa9!bZ\xd8\x91\xeb8$@\xac\xe8",
        id: [ 0x56, 0x14, 0x06, 0x9a, 0x2f, 0xd7, 0xc2, 0xec, 0xd3, 0xf5, 0xe1, 0xbd, 0x44, 0xb2, 0x3e, 0xc7, 0x46, 0x76, 0xb9, 0xbc, 0x99, 0x11, 0x5c, 0xc0, 0xef, 0x94, 0x98, 0x55, 0xd6, 0x89, 0xd0, 0xdd ],
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xd7\xf4\xcci\xb2\xe4\x0e\x90\xa3\x8a\xeaZp\tO\xef\x13b\xd0\x8dI`\xff\x1b@P\x07\x0cmq\x86\xda%I\x8de\xe1\x08\rG4k\xbd\'\xbc\x96!>4\xf5\x87v1\xb1\x7f\x1d\xc9\x85;\r\xf7\x1f?\xe9",
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
        operated_by: "Comodo",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xef\xe4}t.\x15\x15\xb6\xe9\xbb#\x8b\xfb,\xb5\xe1\xc7\x80\x98G\xfb@ih\xfcI\xadaN\x83G<\x1a\xb7\x8d\xdf\xff{0\xb4\xba\xff/\xcb\xa0\x14\xe3\xad\xd5\x85?DY\x8c\x8c`\x8b\xd7\xb8\xb1\xbf\xae\x8cg",
        id: [ 0x6f, 0x53, 0x76, 0xac, 0x31, 0xf0, 0x31, 0x19, 0xd8, 0x99, 0x00, 0xa4, 0x51, 0x15, 0xff, 0x77, 0x15, 0x1c, 0x11, 0xd9, 0x02, 0xc1, 0x00, 0x29, 0x06, 0x8d, 0xb2, 0x08, 0x9a, 0x37, 0xd9, 0x13 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Izenpe log",
     *   "disqualified_at": 1464566400,
     *   "dns_api_endpoint": "izenpe1.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEJ2Q5DC3cUBj4IQCiDu0s6j51up+TZAkAEcQRF6tczw90rLWXkJMAW7jr9yc92bIKgV8vDXU4lDeZHvYHduDuvg==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     4
     *   ],
     *   "url": "ct.izenpe.com/"
     * }
     */
    &sct::Log {
        description: "Izenpe log",
        url: "ct.izenpe.com/",
        operated_by: "Izenpe",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\'d9\x0c-\xdcP\x18\xf8!\x00\xa2\x0e\xed,\xea>u\xba\x9f\x93d\t\x00\x11\xc4\x11\x17\xab\\\xcf\x0ft\xac\xb5\x97\x90\x93\x00[\xb8\xeb\xf7\'=\xd9\xb2\n\x81_/\ru8\x947\x99\x1e\xf6\x07v\xe0\xee\xbe",
        id: [ 0x74, 0x61, 0xb4, 0xa0, 0x9c, 0xfb, 0x3d, 0x41, 0xd7, 0x51, 0x59, 0x57, 0x5b, 0x2e, 0x76, 0x49, 0xa4, 0x45, 0xa8, 0xd2, 0x77, 0x09, 0xb0, 0xcc, 0x56, 0x4a, 0x64, 0x82, 0xb7, 0xeb, 0x41, 0xa3 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "DigiCert Log Server 2",
     *   "dns_api_endpoint": "digicert2.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEzF05L2a4TH/BLgOhNKPoioYCrkoRxvcmajeb8Dj4XQmNY+gxa4Zmz3mzJTwe33i0qMVp+rfwgnliQ/bM/oFmhA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     1
     *   ],
     *   "url": "ct2.digicert-ct.com/log/"
     * }
     */
    &sct::Log {
        description: "DigiCert Log Server 2",
        url: "ct2.digicert-ct.com/log/",
        operated_by: "DigiCert",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xcc]9/f\xb8L\x7f\xc1.\x03\xa14\xa3\xe8\x8a\x86\x02\xaeJ\x11\xc6\xf7&j7\x9b\xf08\xf8]\t\x8dc\xe81k\x86f\xcfy\xb3%<\x1e\xdfx\xb4\xa8\xc5i\xfa\xb7\xf0\x82ybC\xf6\xcc\xfe\x81f\x84",
        id: [ 0x87, 0x75, 0xbf, 0xe7, 0x59, 0x7c, 0xf8, 0x8c, 0x43, 0x99, 0x5f, 0xbd, 0xf3, 0x6e, 0xff, 0x56, 0x8d, 0x47, 0x56, 0x36, 0xff, 0x4a, 0xb5, 0x60, 0xc1, 0xb4, 0xea, 0xff, 0x5e, 0xa0, 0x83, 0x0f ],
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04}\xa8K\x12)\x80\xa3=\xad\xd3Zw\xb8\xcc\xe2\x88\xb3\xa5\xfd\xf1\xd3\x0c\xcd\x18\x0c\xe8AF\xe8\x81\x01\x1b\x15\xe1K\xf1\x1bb\xdd6\n\x08\x18\xba\xed\x0b5\x84\xd0\x9e@<-\x9e\x9b\x82e\xbd\x1f\x04\x10AL\xa0",
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
        key: b"0\x82\x01\"0\r\x06\t*\x86H\x86\xf7\r\x01\x01\x01\x05\x00\x03\x82\x01\x0f\x000\x82\x01\n\x02\x82\x01\x01\x00\xbf\xb5\x08a\x9a)2\x04\xd3%c\xe9\xd8\x85\xe1\x86\xe0\x1f\xd6^\x9a\xf73;\x80\x1b\xe7\xb6>_-\xa1f\xf6\x95J\x84\xa6!Vy\xe8\xf7\x85\xee]\xe3|\x12\xc0\xe0\x89\"\t\">\xba\x16\x95\x06\xbd\xa8\xb9\xb1\xa9\xb2z\xd6a.\x87\x11\xb9x@\x89u\xdb\x0c\xdc\x90\xe0\xa4y\xd6\xd5^n\xd1*\xdb4\xf4\x99?e\x89;F\xc2),\x15\x07\x1c\xc9K\x1aT\xf8l\x1e\xaf`\'b\ne\xd5\x9a\xb9P6\x16nq\xf6\x1f\x01\xf7\x12\xa7\xfc\xbf\xf6!\xa3)\x90\x86-w\xde\xbbL\xd4\xcf\xfd\xd2\xcf\x82,M\xd4\xf2\xc2-\xac\xa9\xbe\xea\xc3\x19%C\xb2\xe5\x9al\r\xc5\x1c\xa5\x8b\xf7?0\xaf\xb9\x01\x91\xb7i\x12\x12\xe5\x83a\xfe4\x00\xbe\xf6q\x8a\xc7\xebP\x92\xe8Y\xfe\x15\x91\xeb\x96\x97\xf8#T?-\x8e\x07\xdf\xee\xda\xb3O\xc8<\x9do\xdf<,CW\xa1G\x0c\x91\x04\xf4uM\xda\x89\x81\xa4\x14\x064\xb9\x98\xc3\xda\xf1\xfd\xed36\xd3\x16-5\x02\x03\x01\x00\x01",
        id: [ 0xa5, 0x77, 0xac, 0x9c, 0xed, 0x75, 0x48, 0xdd, 0x8f, 0x02, 0x5b, 0x67, 0xa2, 0x41, 0x08, 0x9d, 0xf8, 0x6e, 0x0f, 0x47, 0x6e, 0xc2, 0x03, 0xc2, 0xec, 0xbe, 0xdb, 0x18, 0x5f, 0x28, 0x26, 0x38 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Venafi log",
     *   "disqualified_at": 1488307346,
     *   "dns_api_endpoint": "venafi.ct.googleapis.com",
     *   "key": "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAolpIHxdSlTXLo1s6H1OCdpSj/4DyHDc8wLG9wVmLqy1lk9fz4ATVmm+/1iN2Nk8jmctUKK2MFUtlWXZBSpym97M7frGlSaQXUWyA3CqQUEuIJOmlEjKTBEiQAvpfDjCHjlV2Be4qTM6jamkJbiWtgnYPhJL6ONaGTiSPm7Byy57iaz/hbckldSOIoRhYBiMzeNoA0DiRZ9KmfSeXZ1rB8y8X5urSW+iBzf2SaOfzBvDpcoTuAaWx2DPazoOl28fP1hZ+kHUYvxbcMjttjauCFx+JII0dmuZNIwjfeG/GBb9frpSX219k1O4Wi6OEbHEr8at/XQ0y7gTikOxBn/s5wQIDAQAB",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     6
     *   ],
     *   "url": "ctlog.api.venafi.com/"
     * }
     */
    &sct::Log {
        description: "Venafi log",
        url: "ctlog.api.venafi.com/",
        operated_by: "Venafi",
        key: b"0\x82\x01\"0\r\x06\t*\x86H\x86\xf7\r\x01\x01\x01\x05\x00\x03\x82\x01\x0f\x000\x82\x01\n\x02\x82\x01\x01\x00\xa2ZH\x1f\x17R\x955\xcb\xa3[:\x1fS\x82v\x94\xa3\xff\x80\xf2\x1c7<\xc0\xb1\xbd\xc1Y\x8b\xab-e\x93\xd7\xf3\xe0\x04\xd5\x9ao\xbf\xd6#v6O#\x99\xcbT(\xad\x8c\x15KeYvAJ\x9c\xa6\xf7\xb3;~\xb1\xa5I\xa4\x17Ql\x80\xdc*\x90PK\x88$\xe9\xa5\x122\x93\x04H\x90\x02\xfa_\x0e0\x87\x8eUv\x05\xee*L\xce\xa3ji\tn%\xad\x82v\x0f\x84\x92\xfa8\xd6\x86N$\x8f\x9b\xb0r\xcb\x9e\xe2k?\xe1m\xc9%u#\x88\xa1\x18X\x06#3x\xda\x00\xd08\x91g\xd2\xa6}\'\x97gZ\xc1\xf3/\x17\xe6\xea\xd2[\xe8\x81\xcd\xfd\x92h\xe7\xf3\x06\xf0\xe9r\x84\xee\x01\xa5\xb1\xd83\xda\xce\x83\xa5\xdb\xc7\xcf\xd6\x16~\x90u\x18\xbf\x16\xdc2;m\x8d\xab\x82\x17\x1f\x89 \x8d\x1d\x9a\xe6M#\x08\xdfxo\xc6\x05\xbf_\xae\x94\x97\xdb_d\xd4\xee\x16\x8b\xa3\x84lq+\xf1\xab\x7f]\r2\xee\x04\xe2\x90\xecA\x9f\xfb9\xc1\x02\x03\x01\x00\x01",
        id: [ 0xac, 0x3b, 0x9a, 0xed, 0x7f, 0xa9, 0x67, 0x47, 0x57, 0x15, 0x9e, 0x6d, 0x7d, 0x57, 0x56, 0x72, 0xf9, 0xd9, 0x81, 0x00, 0x94, 0x1e, 0x9b, 0xde, 0xff, 0xec, 0xa1, 0x31, 0x3b, 0x75, 0x78, 0x2d ],
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\x12l\x86\x0e\xf6\x17\xb1\x12l7%\xd2\xad\x87=\x0e1\xec!\xad\xb1\xcd\xbe\x14G\xb6qV\x85z\x9a\xb7=\x89\x90{\xc62:\xf8\xda\xce\x8b\x01\xfe?\xfcq\x91\x19\x8e\x14n\x89z]\xb4\xab~\xe1N\x1e|\xac",
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
        operated_by: "Symantec",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\xea\x95\x9e\x02\xff\xee\xf13mK\x87\xbc\xcd\xfd\x19\x17b\xff\x94\xd3\xd0Y\x07?\x02-\x1c\x90\xfe\xc8G0;\xf1\xdd\r\xb8\x11\x0c]\x1d\x86\xdd\xab\xd3+Ff\xfbne\xb7;\xfdYh\xac\xdf\xa6\xf8\xce\xd2\x18M",
        id: [ 0xbc, 0x78, 0xe1, 0xdf, 0xc5, 0xf6, 0x3c, 0x68, 0x46, 0x49, 0x33, 0x4d, 0xa1, 0x0f, 0xa1, 0x5f, 0x09, 0x79, 0x69, 0x20, 0x09, 0xc0, 0x81, 0xb4, 0xf3, 0xf6, 0x91, 0x7f, 0x3e, 0xd9, 0xb8, 0xa5 ],
        max_merge_delay: 86400,
    },

    /*
     * {
     *   "description": "Certly.IO log",
     *   "disqualified_at": 1460678400,
     *   "dns_api_endpoint": "certly.ct.googleapis.com",
     *   "key": "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAECyPLhWKYYUgEc+tUXfPQB4wtGS2MNvXrjwFCCnyYJifBtd2Sk7Cu+Js9DNhMTh35FftHaHu6ZrclnNBKwmbbSA==",
     *   "maximum_merge_delay": 86400,
     *   "operated_by": [
     *     3
     *   ],
     *   "url": "log.certly.io/"
     * }
     */
    &sct::Log {
        description: "Certly.IO log",
        url: "log.certly.io/",
        operated_by: "Certly",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\x0b#\xcb\x85b\x98aH\x04s\xebT]\xf3\xd0\x07\x8c-\x19-\x8c6\xf5\xeb\x8f\x01B\n|\x98&\'\xc1\xb5\xdd\x92\x93\xb0\xae\xf8\x9b=\x0c\xd8LN\x1d\xf9\x15\xfbGh{\xbaf\xb7%\x9c\xd0J\xc2f\xdbH",
        id: [ 0xcd, 0xb5, 0x17, 0x9b, 0x7f, 0xc1, 0xc0, 0x46, 0xfe, 0xea, 0x31, 0x13, 0x6a, 0x3f, 0x8f, 0x00, 0x2e, 0x61, 0x82, 0xfa, 0xf8, 0x89, 0x6f, 0xec, 0xc8, 0xb2, 0xf5, 0xb5, 0xab, 0x60, 0x49, 0x00 ],
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
        operated_by: "Symantec",
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04\x96\xea\xac\x1cF\x0c\x1bU\xdc\r\xfc\xb5\x94\'FWBp:i\x18\xe2\xbf;\xc4\xdb\xab\xa0\xf4\xb6l\xc0S?MB\x103\xf0X\x97\x8fk\xber\xf4*\xec\x1cB\xaa\x03/\x1a~(5v\x99\x08=!\x14\x86",
        id: [ 0xdd, 0xeb, 0x1d, 0x2b, 0x7a, 0x0d, 0x4f, 0xa6, 0x20, 0x8b, 0x81, 0xad, 0x81, 0x68, 0x70, 0x7e, 0x2e, 0x8e, 0x9d, 0x01, 0xd5, 0x5c, 0x88, 0x8d, 0x3d, 0x11, 0xc4, 0xcd, 0xb6, 0xec, 0xbe, 0xcc ],
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
        key: b"0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\x00\x04 [\x18\xc8<\xc1\x8b\xb31\x08\x00\xbf\xa0\x90W+\xb7G\x8co\xb5h\xb0\x8e\x90x\xe9\xa0s\xeaO(!.\x9c\xc0\xf4\x16\x1b\xaa\xf9\xd5\xd7\xa9\x80\xc3N/R<\x98\x01%F$%(#w-\x05\xc2@z",
        id: [ 0xee, 0x4b, 0xbd, 0xb7, 0x75, 0xce, 0x60, 0xba, 0xe1, 0x42, 0x69, 0x1f, 0xab, 0xe1, 0x9e, 0x66, 0xa3, 0x0f, 0x7e, 0x5f, 0xb0, 0x72, 0xd8, 0x83, 0x00, 0xc4, 0x7b, 0x89, 0x7a, 0xa8, 0xfd, 0xcb ],
        max_merge_delay: 86400,
    },

];
