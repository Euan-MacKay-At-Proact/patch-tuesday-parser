use crate::data_structure::CsrfDoc;
use std::fmt;

pub struct Report {
    title: String,

    total_vulns: u16,
    total_disclosed_vulns: u16,
    total_exploited_vulns: u16,
    total_critical_vulns: u16,

    exploited_vulns: Vec<Vuln>,
    disclosed_vulns: Vec<Vuln>,
}

impl Report {
    pub fn to_html(&self) -> String {
        let mut s = format!(
            "<h1>Microsoft - Multiple Products - {}</h1>\
            <a href=\"https://teams.microsoft.com/l/team/19%3azyL0LCtHqHzphE7eObcnrarbHFspb5URbpRH6nSt6U01%40thread.tacv2/conversations?groupId=407e1952-854e-4b4b-94ec-d6064405b99a&tenantId=c047d5c1-d2a6-484d-ae6e-ec2bce85d023\"Critical Vulnerability Response - </a><br>\
            <p>{} addresses a total of {} vulnerabilities, with \
            {} classified as 'Critical', \
            {} being actively exploited, and \
            {} publicly disclosed.</p><br>\
            <h2>Actively Exploited</h2><ul>",
            self.title,
            self.title,
            self.total_vulns,
            self.total_critical_vulns,
            self.total_exploited_vulns,
            self.total_disclosed_vulns,
        );
        for vuln in &self.exploited_vulns {
            s.push_str(format!("<li>{}</li>", vuln).as_str());
        }
        s.push_str(format!("</ul><br><h2>Publicly Disclosed Vulnerabilities</h2><ul>").as_str());
        for vuln in &self.disclosed_vulns {
            s.push_str(format!("<li>{}</li>", vuln).as_str());
        }
        s.push_str(format!(
            "</ul><br><p>For a full breakdown of all critical vulnerabilities, vulnerabilities \
            in need of urgent patching, vulnerabilities that are being exploited in the wild, \
            and vulnerabilities that have been publicly disclosed, see the attached spreadsheet.</p>"
        ).as_str());

        s
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n\
            [+] Found a total of {} vulnerabilities\n\
            [+] {} disclosed vulnerabilities\n\
            [+] {} exploited vulnerabilities\n\
            [+] {} in need of urgent patching\n",
            self.title,
            self.total_vulns,
            self.total_disclosed_vulns,
            self.total_exploited_vulns,
            self.total_critical_vulns
        )
    }
}

struct Vuln {
    cve: String,
    cvss: f32,
    name: String,
}

impl fmt::Display for Vuln {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | CVSS: {} | {}", self.cve, self.cvss, self.name)
    }
}

pub fn generate_report(doc: CsrfDoc) -> Report {
    let mut report = Report {
        title: String::new(),
        total_vulns: 0,
        total_disclosed_vulns: 0,
        total_exploited_vulns: 0,
        total_critical_vulns: 0,
        exploited_vulns: Vec::new(),
        disclosed_vulns: Vec::new(),
    };

    if let Some(s) = doc.document_title.value {
        report.title = format!(
            "Patch Tuesday {}",
            s.strip_suffix("Security Updates").unwrap()
        );
    }

    report.disclosed_vulns = Vec::new();
    for vuln in doc.vulnerability {
        report.total_vulns += 1;

        if vuln.release_date_specified {
            report.total_disclosed_vulns += 1;

            let v = Vuln {
                cve: vuln.cve.clone(),
                cvss: vuln.cvss_score_sets[0].base_score.clone(),
                name: vuln.title.value.clone().unwrap(),
            };

            report.disclosed_vulns.push(v);
        }

        report.exploited_vulns = Vec::new();
        for threat in vuln.threats {
            if let Some(_s) = threat.description.value.unwrap().find("Exploited:Yes") {
                report.total_exploited_vulns += 1;

                let v = Vuln {
                    cve: vuln.cve.clone(),
                    cvss: vuln.cvss_score_sets[0].base_score.clone(),
                    name: vuln.title.value.clone().unwrap(),
                };

                report.exploited_vulns.push(v);

                break;
            }
        }

        if vuln.cvss_score_sets[0].base_score >= 8.0 {
            report.total_critical_vulns += 1;
        }
    }

    report
}
