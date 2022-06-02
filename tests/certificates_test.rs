use crate::common::setup_mock_for;
use dnsimple::dnsimple::certificates::{
    LetsEncryptPurchasePayload, LetsEncryptPurchaseRenewalPayload,
};
mod common;

#[test]
fn test_list_certificates() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates",
        "listCertificates/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let response = client
        .certificates()
        .list_certificates(account_id, domain, None)
        .unwrap();
    let certificates = response.data.unwrap();

    assert_eq!(2, certificates.len());

    let certificate = certificates.first().unwrap();

    assert_eq!(101973, certificate.id);
    assert_eq!(14279, certificate.domain_id);
    assert_eq!(11435, certificate.contact_id);
    assert_eq!("www2", certificate.name);
    assert_eq!("www2.dnsimple.us", certificate.common_name);
    assert_eq!(1, certificate.years);
    assert_eq!("-----BEGIN CERTIFICATE REQUEST-----\nMIICYDCCAUgCAQAwGzEZMBcGA1UEAwwQd3d3Mi5kbnNpbXBsZS51czCCASIwDQYJ\nKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMjXrephLTu7OKVQ6F3LhmLkL6NL3ier\n1qaWPtJBbkBuzJIn8gmSG+6xGmywB6GKvP2IVkPQhPBpfc8wsTd26rbSBHnRIQal\ntk+W4aQZyIeXFARY+cRvpjeAtmpX0vwZkDMoEyhFomBfGxVfx6tSqdGlR88/x0By\ny5u7+xwkY+4jMt+wZi+wpXsScumB6DAC1PTYRvNFQy7Gcjqrc3EdzPsn3c9kLCNO\n3GCPJoWmT5Rtyd7FxjJiSIf7BDOi12BnblpSLwGvtu6Wrl+u9LJLj8zeCACwUiQG\nuvnP2lAl2YacNAgpql6C2eEnFjIub7Ul1QMUImQSDVy5dMd/UGQrOb0CAwEAAaAA\nMA0GCSqGSIb3DQEBCwUAA4IBAQA8oVxOrZCGeSFmKpNV4oilzPOepTVSWxXa19T7\nzD/azh6j6RBLZPpG4TFbpvjecum+1V7Y8ypIcwhRtlh5/zSbfJkjJsdCdZU9XZat\nT5YkOaxuCUCDajpRiyyKhHvrloTPKPXe5ygCq/Q23xm//VrXKArLSWVB9qWS6gDV\nk0y3/mIlTQ3mTgfYQySc3MPXvIgUoqmB8Ajfq1n3hSLgb1/OoKNfeVEWsON116cq\nbXvl63+XzPubj6KWZXZH/jhrs53fuLq3xyeeuOaPrn+2VceBVt4DCC9n0JS5wepl\nHDoVxtWTTNeJdP5xFB5V1KI+D4FEFBUGnQABEvajpU3vljh3\n-----END CERTIFICATE REQUEST-----\n",
                                    certificate.csr.to_owned().unwrap());
    assert_eq!("issued", certificate.state);
    assert_eq!(false, certificate.auto_renew);
    assert!(certificate.alternate_names.is_empty());
    assert_eq!("letsencrypt", certificate.authority_identifier);
    assert_eq!("2020-06-18T20:15:09Z", certificate.created_at);
    assert_eq!("2020-06-18T20:30:08Z", certificate.updated_at);
    assert_eq!(
        "2020-09-16T19:30:07Z",
        certificate.expires_at.to_owned().unwrap()
    );
    assert_eq!("2020-09-16", certificate.expires_on.to_owned().unwrap());
}

#[test]
fn test_get_certificate() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/101967",
        "getCertificate/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;

    let certificate = client
        .certificates()
        .get_certificate(account_id, domain, certificate_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(101967, certificate.id);
    assert_eq!(289333, certificate.domain_id);
    assert_eq!(2511, certificate.contact_id);
    assert_eq!("www", certificate.name);
    assert_eq!("www.bingo.pizza", certificate.common_name);
    assert_eq!(1, certificate.years);
    assert_eq!("-----BEGIN CERTIFICATE REQUEST-----\nMIICmTCCAYECAQAwGjEYMBYGA1UEAwwPd3d3LmJpbmdvLnBpenphMIIBIjANBgkq\nhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAw4+KoZ9IDCK2o5qAQpi+Icu5kksmjQzx\n5o5g4B6XhRxhsfHlK/i3iU5hc8CONjyVv8j82835RNsiKrflnxGa9SH68vbQfcn4\nIpbMz9c+Eqv5h0Euqlc3A4DBzp0unEu5QAUhR6Xu1TZIWDPjhrBOGiszRlLQcp4F\nzy6fD6j5/d/ylpzTp5v54j+Ey31Bz86IaBPtSpHI+Qk87Hs8DVoWxZk/6RlAkyur\nXDGWnPu9n3RMfs9ag5anFhggLIhCNtVN4+0vpgPQ59pqwYo8TfdYzK7WSKeL7geu\nCqVE3bHAqU6dLtgHOZfTkLwGycUh4p9aawuc6fsXHHYDpIL8s3vAvwIDAQABoDow\nOAYJKoZIhvcNAQkOMSswKTAnBgNVHREEIDAeggtiaW5nby5waXp6YYIPd3d3LmJp\nbmdvLnBpenphMA0GCSqGSIb3DQEBCwUAA4IBAQBwOLKv+PO5hSJkgqS6wL/wRqLh\nQ1zbcHRHAjRjnpRz06cDvN3X3aPI+lpKSNFCI0A1oKJG7JNtgxX3Est66cuO8ESQ\nPIb6WWN7/xlVlBCe7ZkjAFgN6JurFdclwCp/NI5wBCwj1yb3Ar5QQMFIZOezIgTI\nAWkQSfCmgkB96d6QlDWgidYDDjcsXugQveOQRPlHr0TsElu47GakxZdJCFZU+WPM\nodQQf5SaqiIK2YaH1dWO//4KpTS9QoTy1+mmAa27apHcmz6X6+G5dvpHZ1qH14V0\nJoMWIK+39HRPq6mDo1UMVet/xFUUrG/H7/tFlYIDVbSpVlpVAFITd/eQkaW/\n-----END CERTIFICATE REQUEST-----\n",
               certificate.csr.to_owned().unwrap());
    assert_eq!("issued", certificate.state);
    assert_eq!(false, certificate.auto_renew);
    assert!(certificate.alternate_names.is_empty());
    assert_eq!("letsencrypt", certificate.authority_identifier);
    assert_eq!("2020-06-18T18:54:17Z", certificate.created_at);
    assert_eq!("2020-06-18T19:10:14Z", certificate.updated_at);
    assert_eq!(
        "2020-09-16T18:10:13Z",
        certificate.expires_at.to_owned().unwrap()
    );
    assert_eq!("2020-09-16", certificate.expires_on.to_owned().unwrap());
}

#[test]
fn test_download_certificate() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/101967/download",
        "downloadCertificate/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;

    let download = client
        .certificates()
        .download_certificate(account_id, domain, certificate_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!("-----BEGIN CERTIFICATE-----\nMIIE7TCCA9WgAwIBAgITAPpTe4O3vjuQ9L4gLsogi/ukujANBgkqhkiG9w0BAQsF\nADAiMSAwHgYDVQQDDBdGYWtlIExFIEludGVybWVkaWF0ZSBYMTAeFw0xNjA2MTEx\nNzQ4MDBaFw0xNjA5MDkxNzQ4MDBaMBkxFzAVBgNVBAMTDnd3dy53ZXBwb3MubmV0\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtzCcMfWoQRt5AMEY0HUb\n2GaraL1GsWOo6YXdPfe+YDvtnmDw23NcoTX7VSeCgU9M3RKs19AsCJcRNTLJ2dmD\nrAuyCTud9YTAaXQcTOLUhtO8T8+9AFVIva2OmAlKCR5saBW3JaRxW7V2aHEd/d1s\ns1CvNOO7jNppc9NwGSnDHcn3rqNv/U3MaU0gpJJRqsKkvcLU6IHJGgxyQ6AbpwJD\nIqBnzkjHu2IuhGEbRuMjyWLA2qtsjyVlfPotDxUdVouUQpz7dGHUFrLR7ma8QAYu\nOfl1ZMyrc901HGMa7zwbnFWurs3fed7vAosTRZIjnn72/3Wo7L9RiMB+vwr3NX7c\n9QIDAQABo4ICIzCCAh8wDgYDVR0PAQH/BAQDAgWgMB0GA1UdJQQWMBQGCCsGAQUF\nBwMBBggrBgEFBQcDAjAMBgNVHRMBAf8EAjAAMB0GA1UdDgQWBBRh9q/3Zxbk4yA/\nt7j+8xA+rkiZBTAfBgNVHSMEGDAWgBTAzANGuVggzFxycPPhLssgpvVoOjB4Bggr\nBgEFBQcBAQRsMGowMwYIKwYBBQUHMAGGJ2h0dHA6Ly9vY3NwLnN0Zy1pbnQteDEu\nbGV0c2VuY3J5cHQub3JnLzAzBggrBgEFBQcwAoYnaHR0cDovL2NlcnQuc3RnLWlu\ndC14MS5sZXRzZW5jcnlwdC5vcmcvMCUGA1UdEQQeMByCCndlcHBvcy5uZXSCDnd3\ndy53ZXBwb3MubmV0MIH+BgNVHSAEgfYwgfMwCAYGZ4EMAQIBMIHmBgsrBgEEAYLf\nEwEBATCB1jAmBggrBgEFBQcCARYaaHR0cDovL2Nwcy5sZXRzZW5jcnlwdC5vcmcw\ngasGCCsGAQUFBwICMIGeDIGbVGhpcyBDZXJ0aWZpY2F0ZSBtYXkgb25seSBiZSBy\nZWxpZWQgdXBvbiBieSBSZWx5aW5nIFBhcnRpZXMgYW5kIG9ubHkgaW4gYWNjb3Jk\nYW5jZSB3aXRoIHRoZSBDZXJ0aWZpY2F0ZSBQb2xpY3kgZm91bmQgYXQgaHR0cHM6\nLy9sZXRzZW5jcnlwdC5vcmcvcmVwb3NpdG9yeS8wDQYJKoZIhvcNAQELBQADggEB\nAEqMdWrmdIyQxthWsX3iHmM2h/wXwEesD0VIaA+Pq4mjwmKBkoPSmHGQ/O4v8RaK\nB6gl8v+qmvCwwqC1SkBmm+9C2yt/P6WhAiA/DD+WppYgJWfcz2lEKrgufFlHPukB\nDzE0mJDuXm09QTApWlaTZWYfWKY50T5uOT/rs+OwGFFCO/8o7v5AZRAHos6uzjvq\nAtFZj/FEnXXMjSSlQ7YKTXToVpnAYH4e3/UMsi6/O4orkVz82ZfhKwMWHV8dXlRw\ntQaemFWTjGPgSLXJAtQO30DgNJBHX/fJEaHv6Wy8TF3J0wOGpzGbOwaTX8YAmEzC\nlzzjs+clg5MN5rd1g4POJtU=\n-----END CERTIFICATE-----\n",
                download.server);
    assert_eq!(None, download.root);
    let chain = download.chain;
    assert_eq!(1, chain.len());
    assert_eq!("-----BEGIN CERTIFICATE-----\nMIIEqzCCApOgAwIBAgIRAIvhKg5ZRO08VGQx8JdhT+UwDQYJKoZIhvcNAQELBQAw\nGjEYMBYGA1UEAwwPRmFrZSBMRSBSb290IFgxMB4XDTE2MDUyMzIyMDc1OVoXDTM2\nMDUyMzIyMDc1OVowIjEgMB4GA1UEAwwXRmFrZSBMRSBJbnRlcm1lZGlhdGUgWDEw\nggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDtWKySDn7rWZc5ggjz3ZB0\n8jO4xti3uzINfD5sQ7Lj7hzetUT+wQob+iXSZkhnvx+IvdbXF5/yt8aWPpUKnPym\noLxsYiI5gQBLxNDzIec0OIaflWqAr29m7J8+NNtApEN8nZFnf3bhehZW7AxmS1m0\nZnSsdHw0Fw+bgixPg2MQ9k9oefFeqa+7Kqdlz5bbrUYV2volxhDFtnI4Mh8BiWCN\nxDH1Hizq+GKCcHsinDZWurCqder/afJBnQs+SBSL6MVApHt+d35zjBD92fO2Je56\ndhMfzCgOKXeJ340WhW3TjD1zqLZXeaCyUNRnfOmWZV8nEhtHOFbUCU7r/KkjMZO9\nAgMBAAGjgeMwgeAwDgYDVR0PAQH/BAQDAgGGMBIGA1UdEwEB/wQIMAYBAf8CAQAw\nHQYDVR0OBBYEFMDMA0a5WCDMXHJw8+EuyyCm9Wg6MHoGCCsGAQUFBwEBBG4wbDA0\nBggrBgEFBQcwAYYoaHR0cDovL29jc3Auc3RnLXJvb3QteDEubGV0c2VuY3J5cHQu\nb3JnLzA0BggrBgEFBQcwAoYoaHR0cDovL2NlcnQuc3RnLXJvb3QteDEubGV0c2Vu\nY3J5cHQub3JnLzAfBgNVHSMEGDAWgBTBJnSkikSg5vogKNhcI5pFiBh54DANBgkq\nhkiG9w0BAQsFAAOCAgEABYSu4Il+fI0MYU42OTmEj+1HqQ5DvyAeyCA6sGuZdwjF\nUGeVOv3NnLyfofuUOjEbY5irFCDtnv+0ckukUZN9lz4Q2YjWGUpW4TTu3ieTsaC9\nAFvCSgNHJyWSVtWvB5XDxsqawl1KzHzzwr132bF2rtGtazSqVqK9E07sGHMCf+zp\nDQVDVVGtqZPHwX3KqUtefE621b8RI6VCl4oD30Olf8pjuzG4JKBFRFclzLRjo/h7\nIkkfjZ8wDa7faOjVXx6n+eUQ29cIMCzr8/rNWHS9pYGGQKJiY2xmVC9h12H99Xyf\nzWE9vb5zKP3MVG6neX1hSdo7PEAb9fqRhHkqVsqUvJlIRmvXvVKTwNCP3eCjRCCI\nPTAvjV+4ni786iXwwFYNz8l3PmPLCyQXWGohnJ8iBm+5nk7O2ynaPVW0U2W+pt2w\nSVuvdDM5zGv2f9ltNWUiYZHJ1mmO97jSY/6YfdOUH66iRtQtDkHBRdkNBsMbD+Em\n2TgBldtHNSJBfB3pm9FblgOcJ0FSWcUDWJ7vO0+NTXlgrRofRT6pVywzxVo6dND0\nWzYlTWeUVsO40xJqhgUQRER9YLOLxJ0O6C8i0xFxAMKOtSdodMB3RIwt7RFQ0uyt\nn5Z5MqkYhlMI3J1tPRTp1nEt9fyGspBOO05gi148Qasp+3N+svqKomoQglNoAxU=\n-----END CERTIFICATE-----",
            chain.first().unwrap());
}
#[test]
fn test_get_certificate_private_key() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/101967/private_key",
        "getCertificatePrivateKey/success",
        "GET",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;

    let certificate_private_key = client
        .certificates()
        .get_certificate_private_key(account_id, domain, certificate_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!("-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEAtzCcMfWoQRt5AMEY0HUb2GaraL1GsWOo6YXdPfe+YDvtnmDw\n23NcoTX7VSeCgU9M3RKs19AsCJcRNTLJ2dmDrAuyCTud9YTAaXQcTOLUhtO8T8+9\nAFVIva2OmAlKCR5saBW3JaRxW7V2aHEd/d1ss1CvNOO7jNppc9NwGSnDHcn3rqNv\n/U3MaU0gpJJRqsKkvcLU6IHJGgxyQ6AbpwJDIqBnzkjHu2IuhGEbRuMjyWLA2qts\njyVlfPotDxUdVouUQpz7dGHUFrLR7ma8QAYuOfl1ZMyrc901HGMa7zwbnFWurs3f\ned7vAosTRZIjnn72/3Wo7L9RiMB+vwr3NX7c9QIDAQABAoIBAEQx32OlzK34GTKT\nr7Yicmw7xEGofIGa1Q2h3Lut13whsxKLif5X0rrcyqRnoeibacS+qXXrJolIG4rP\nTl8/3wmUDQHs5J+6fJqFM+fXZUCP4AFiFzzhgsPBsVyd0KbWYYrZ0qU7s0ttoRe+\nTGjuHgIe3ip1QKNtx2Xr50YmytDydknmro79J5Gfrub1l2iA8SDm1eBrQ4SFaNQ2\nU709pHeSwX8pTihUX2Zy0ifpr0O1wYQjGLneMoG4rrNQJG/z6iUdhYczwwt1kDRQ\n4WkM2sovFOyxbBfoCQ3Gy/eem7OXfjNKUe47DAVLnPkKbqL/3Lo9FD7kcB8K87Ap\nr/vYrl0CgYEA413RAk7571w5dM+VftrdbFZ+Yi1OPhUshlPSehavro8kMGDEG5Ts\n74wEz2X3cfMxauMpMrBk/XnUCZ20AnWQClK73RB5fzPw5XNv473Tt/AFmt7eLOzl\nOcYrhpEHegtsD/ZaljlGtPqsjQAL9Ijhao03m1cGB1+uxI7FgacdckcCgYEAzkKP\n6xu9+WqOol73cnlYPS3sSZssyUF+eqWSzq2YJGRmfr1fbdtHqAS1ZbyC5fZVNZYV\nml1vfXi2LDcU0qS04JazurVyQr2rJZMTlCWVET1vhik7Y87wgCkLwKpbwamPDmlI\n9GY+fLNEa4yfAOOpvpTJpenUScxyKWH2cdYFOOMCgYBhrJnvffINC/d64Pp+BpP8\nyKN+lav5K6t3AWd4H2rVeJS5W7ijiLTIq8QdPNayUyE1o+S8695WrhGTF/aO3+ZD\nKQufikZHiQ7B43d7xL7BVBF0WK3lateGnEVyh7dIjMOdj92Wj4B6mv2pjQ2VvX/p\nAEWVLCtg24/+zL64VgxmXQKBgGosyXj1Zu2ldJcQ28AJxup3YVLilkNje4AXC2No\n6RCSvlAvm5gpcNGE2vvr9lX6YBKdl7FGt8WXBe/sysNEFfgmm45ZKOBCUn+dHk78\nqaeeQHKHdxMBy7utZWdgSqt+ZS299NgaacA3Z9kVIiSLDS4V2VeW7riujXXP/9TJ\nnxaRAoGBAMWXOfNVzfTyrKff6gvDWH+hqNICLyzvkEn2utNY9Q6WwqGuY9fvP/4Z\nXzc48AOBzUr8OeA4sHKJ79sJirOiWHNfD1swtvyVzsFZb6moiNwD3Ce/FzYCa3lQ\nU8blTH/uqpR2pSC6whzJ/lnSdqHUqhyp00000000000000000000\n-----END RSA PRIVATE KEY-----\n",
        certificate_private_key.private_key);
}

#[test]
fn test_purchase_letsencrypt_certificate() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/letsencrypt",
        "purchaseLetsencryptCertificate/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";

    let payload = LetsEncryptPurchasePayload {
        auto_renew: false,
        name: String::from("test-certificate"),
        alternate_names: vec![],
    };

    let letsencrypt = client
        .certificates()
        .purchase_letsencrypt_certificate(account_id, domain, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(101967, letsencrypt.id);
    assert_eq!(101967, letsencrypt.certificate_id);
    assert_eq!("new", letsencrypt.state);
    assert_eq!(false, letsencrypt.auto_renew);
    assert_eq!("2020-06-18T18:54:17Z", letsencrypt.created_at);
    assert_eq!("2020-06-18T18:54:17Z", letsencrypt.updated_at);
}

#[test]
fn test_issue_letsencrypt_certificate() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/letsencrypt/101967/issue",
        "issueLetsencryptCertificate/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;

    let certificate = client
        .certificates()
        .issue_letsencrypt_certificate(account_id, domain, certificate_id)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(101967, certificate.id);
    assert_eq!(289333, certificate.domain_id);
    assert_eq!(2511, certificate.contact_id);
    assert_eq!("www", certificate.name);
    assert_eq!("www.bingo.pizza", certificate.common_name);
    assert_eq!(1, certificate.years);
    assert_eq!(None, certificate.csr);
    assert_eq!("requesting", certificate.state);
    assert_eq!(false, certificate.auto_renew);
    assert!(certificate.alternate_names.is_empty());
    assert_eq!("letsencrypt", certificate.authority_identifier);
    assert_eq!("2020-06-18T18:54:17Z", certificate.created_at);
    assert_eq!("2020-06-18T18:56:20Z", certificate.updated_at);
    assert_eq!(None, certificate.expires_at);
    assert_eq!(None, certificate.expires_on);
}

#[test]
fn test_purchase_letsencrypt_certificate_renewal() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/letsencrypt/101967/renewals",
        "purchaseRenewalLetsencryptCertificate/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;

    let payload = LetsEncryptPurchaseRenewalPayload { auto_renew: false };

    let letsencrypt_renewal = client
        .certificates()
        .purchase_letsencrypt_certificate_renewal(account_id, domain, certificate_id, payload)
        .unwrap()
        .data
        .unwrap();

    assert_eq!(65082, letsencrypt_renewal.id);
    assert_eq!(101967, letsencrypt_renewal.old_certificate_id);
    assert_eq!(101972, letsencrypt_renewal.new_certificate_id);
    assert_eq!("new", letsencrypt_renewal.state);
    assert_eq!(false, letsencrypt_renewal.auto_renew);
    assert_eq!("2020-06-18T19:56:20Z", letsencrypt_renewal.created_at);
    assert_eq!("2020-06-18T19:56:20Z", letsencrypt_renewal.updated_at);
}

#[test]
fn test_issue_letsencrypt_certificate_renewal() {
    let setup = setup_mock_for(
        "/1385/domains/example.com/certificates/letsencrypt/101967/renewals/12121/issue",
        "issueRenewalLetsencryptCertificate/success",
        "POST",
    );
    let client = setup.0;
    let account_id = 1385;
    let domain = "example.com";
    let certificate_id = 101967;
    let certificate_renewal_id = 12121;

    let renewal = client
        .certificates()
        .issue_letsencrypt_certificate_renewal(
            account_id,
            domain,
            certificate_id,
            certificate_renewal_id,
        )
        .unwrap()
        .data
        .unwrap();

    assert_eq!(101972, renewal.id);
    assert_eq!(289333, renewal.domain_id);
    assert_eq!(2511, renewal.contact_id);
    assert_eq!("www", renewal.name);
    assert_eq!("www.bingo.pizza", renewal.common_name);
    assert_eq!(1, renewal.years);
    assert_eq!(None, renewal.csr);
    assert_eq!("requesting", renewal.state);
    assert_eq!(false, renewal.auto_renew);
    assert!(renewal.alternate_names.is_empty());
    assert_eq!("letsencrypt", renewal.authority_identifier);
    assert_eq!("2020-06-18T19:56:20Z", renewal.created_at);
    assert_eq!("2020-06-18T20:05:26Z", renewal.updated_at);
    assert_eq!(None, renewal.expires_at);
    assert_eq!(None, renewal.expires_on);
}
