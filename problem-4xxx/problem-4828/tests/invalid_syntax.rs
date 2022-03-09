use problem_4828::{Error, TokenStream, Xml};

#[test]
fn tag_without_closing() {
    let source = "<doc>the quick brown fox.";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::InvalidTagName));
}

#[test]
fn mixed_tags() {
    let source = "the <i>quick <b>brown</i></b> fox";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Err(Error::NotEqualTagName("b".to_string(), "i".to_string())),
    );
}

#[test]
fn text_with_unexpected_escaping() {
    let source = "fox & socks.";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::InvalidChar('&')));
}

#[test]
fn text_with_unexpected_chevron() {
    let source = "3x+5>7";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::LoopDetected));
}

#[test]
fn text_with_unexpected_hex() {
    let source = "Null: &x0;";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::InvalidChar('&')));
}

#[test]
fn text_with_camel_cased_tag() {
    let source = "tags that are not lowercase <Tag1>hello</Tag1>";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::InvalidTagName));
}

#[test]
fn weired() {
    let source = "<7><s><9ip70><jjl6><n3a1><miq4gr><cq><7ba0><dv604><cfc><9qnnf><bi/>&amp;8b&gt;8&lt;&xfr22;&gt;&amp;</9qnnf>&x2e9a1e1f61;</cfc>&x088D27;&lt;&x72y6bc0658;&lt;&amp;<mn/></dv604>&x6d066D6d52;&amp;</7ba0><3l/></cq><u979j/>&xc5;&x62E6;</miq4gr><ng/><6ew9k/><ux0mx/>&xAaB2526873;7<3aud/> &xf92A7jer;&amp;&lt;&amp;&amp;<xu2dur>&x6[8F-9HBAb;&gt;<6bo></6bo>&x2F;s<saqlev/></xu2dur>%<vlj5dq/><2xe71n/>&x047281;<ck9vf/>C&xF8166801a0;&x2EE668;A</n3a1><pur/>&x9e4D;&xCA;&amp;</jjl6><q5/>&xD1500C;</9ip70></s>&amp;&amp;&amp;</7><fh/>&gt;";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Err(Error::InvalidHex('r')));
}
