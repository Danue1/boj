use problem_4828::{Element, Error, Name, Tag, Text, TokenStream, Xml};

#[test]
fn empty() {
    let source = "";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(xml, Ok(Xml { children: vec![] }));
}

#[test]
fn text() {
    let source = "the quick brown fox.";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Text(Text {
                raw: "the quick brown fox.".to_string(),
            })],
        }),
    );
}

#[test]
fn tag() {
    let source = "<a>the quick brown fox.</a>";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Tag(Tag {
                name: Name {
                    raw: "a".to_string(),
                },
                children: vec![Element::Text(Text {
                    raw: "the quick brown fox.".to_string(),
                })],
            })],
        }),
    );
}

#[test]
fn self_closing_tag() {
    let source = "<a/>";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Tag(Tag {
                name: Name {
                    raw: "a".to_string(),
                },
                children: vec![],
            })],
        }),
    );
}

#[test]
fn tag_with_escape() {
    let source = "<doc>fox &amp; socks.</doc>";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Tag(Tag {
                name: Name {
                    raw: "doc".to_string(),
                },
                children: vec![Element::Text(Text {
                    raw: "fox & socks.".to_string(),
                })],
            })],
        }),
    );
}

#[test]
fn text_with_escape() {
    let source = "3x+5&gt;7";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Text(Text {
                raw: "3x+5>7".to_string(),
            })],
        }),
    );
}

#[test]
fn text_with_hex() {
    let source = "Null: &x00;";
    let xml = Xml::parse(&mut TokenStream::new(source));

    assert_eq!(
        xml,
        Ok(Xml {
            children: vec![Element::Text(Text {
                raw: "Null: 00".to_string(),
            })],
        }),
    );
}

#[test]
fn long() {
    let source = r#"<ley1kmf2f0><kgfy7z><579sugq/>A<fyu27ko><o7sv/>&lt;<8y568k8m></8y568k8m>&x0F77d2;&x3Bf39B;</fyu27ko>'<jns6z05/>&gt;&amp;</kgfy7z>&xb02c69;&xaCfAD5637b;&x4d78;<w6eum9qf4/>&xd83E57;<z6fl1i></z6fl1i>&gt;g&amp;&lt;P<coocdm/>\<u764/>R<nwuf1/>&lt;<mtazs/>G&lt;&amp;</ley1kmf2f0>&lt;<faz><a9bmr69xq></a9bmr69xq>&lt;&xD099;R</faz>?&x45;&gt;<e11q1><9iovms></9iovms>8</e11q1>&lt;<4j/>&gt;&amp;3&gt;&x77;&x297f;&xa30a;7&x74698C76;<z2yn1o><l56woyv61><e7x2k5wee5><pub05o><c><ouyg><7jgm4u8slh><cg4qf><vk><8k02g6qli><e5zhqpcj><k6o41tcy8u><6935r7kbk2><fvl/>&gt;&gt;&x5394;&amp;<p6s/>Rtl<6b7lev></6b7lev><p6843ilz2></p6843ilz2>&xF491;B&lt;&lt;<aqebjejs/>/</6935r7kbk2>&lt;&lt;<vc4v3u13/><q1z/>&lt;</k6o41tcy8u>&gt;<pa1z09/><h8gznh/></e5zhqpcj></8k02g6qli>&x9E;&x2A;&gt;</vk>&x007c;</cg4qf><1fgcezqxu0/>c</7jgm4u8slh><u5txcex><1q></1q>[&lt;6&xdC468C02D3;&gt;<q2tbabw2r></q2tbabw2r>&x7860;<02i4wv/><ixvisflajp/>o&x6DF97A;</u5txcex>&lt;&gt;<mvjd1hqz/>&lt;&xfc;<9fu9>&x2f3323;<r7rg7/><n4o/><httyft></httyft>`</9fu9><o/>w<dqgu/><416g/><fodpnva1u/>&xc7DBE58e92;~&amp;&xc14FF56B;<ul><6wm8fjq9x4/></ul><1t9ekmm5/>&x6668;<t7bo8/>&xb8c918;&lt;&x85Ec32D333;</ouyg>&gt;&xe52F;</c></pub05o><n/>(</e7x2k5wee5></l56woyv61>:&lt;&xb2;&amp;</z2yn1o>&amp;<ld2/>&x9605f5;&x6226;<olx9></olx9>&xad3B38;<p7hyo6>&amp;&amp;&gt;</p7hyo6><79prb/>&x8f44cf;&xc93aADd3;&amp;&xeE;<u1p675/>&amp;&amp;&amp;&x17C6;<f5g8pd><o59>&x856e37cb;</o59>k<e/></f5g8pd>HO&x30aFe5d6F8;&amp;<13dr/>&amp;&lt;<6pw9w4lyqh>&amp;<ies0okxqv/>&amp;&gt;&amp;&x50A2;&x4463862c;&amp;<zrlpu403u/>b</6pw9w4lyqh>:+<m6ue029y4><7><rmai><o8yp6p0wt4/><16cw/><otf2><g><lshd></lshd></g><46/>t&xd266;</otf2><ky9ew72vui/>!&lt;<5jqj317o/>&amp;'&gt;<os8/><lif3fmicne></lif3fmicne>&x3850B41C;<qvqbw498/>&xe585D6EA;<8ltwka0au/><zrt4/>&lt;&x3972A731;&gt;<r/>&amp;&gt;W<bxnn3me3/>&xA3;</rmai><cebmwd><p2dja>&x118B16;&amp;<53owamf/>&x02;<7><jhvmkwo></jhvmkwo>R<08td8ppw></08td8ppw><dq/>Y</7><3xxb/>&amp;&lt;&x9C;<j><lw1b><q690ht84sy></q690ht84sy>&xC1;<w9pc2v3></w9pc2v3><u2x8k84ilx></u2x8k84ilx>$_&gt;V<sl6sn><6tb1q></6tb1q></sl6sn>&x42;<jmu/><5x/><l5rl8/><gud7abql/></lw1b>v<l0ax8i61></l0ax8i61>?&x68E4;&xD06600;<jab62hj3q/>p&x00C8;</j><7virwvd/>f&lt;&xA77e76;</p2dja></cebmwd><o/><m6iz3ht8w><7h1r08/></m6iz3ht8w>^&gt;<l/>&lt;^<w8j/>S&amp;&amp;0&x80eA;</7><f1496m4eaq/>&gt;[w'&xc00578aa;b<186fo/>&lt;<rxr0/>Z<fes/></m6ue029y4>&gt;&xD6;<95sibm><thxo><th8xo><ak906t><lmk1><tyrbf><v3yuv><s9l5k></s9l5k>&x1b6871FacB;<2g>&x1165B6709d;5</2g><dolb57e1nr/>&x54A1;</v3yuv><bzrj/><q/></tyrbf><1><a61cjz/></1><39v2f2wlmb/>&lt;&x0251Aa;&xCc2f;&xf7;&x6d479E;<sbq3mfu/><7v8g5/>&x19;+<55j>&x7Eb959;</55j><a/>&xf97e26;&x5A97b20df9;&x3a9c;&amp;&xD4;&xB481F5;<kqd6h>&x02f1e1fa;<djg9/>|&lt;&lt;&x7f;t&x69d6fe18;;<jmx/><vwtc8/><m3s6e69ge></m3s6e69ge>k<r><9198f></9198f>&amp;&amp;</r><58dw2/><6l><c2a></c2a>&xFc1FFba2;&lt;&amp;{<gwj2agb5/><mmzom7mq9><a69xy></a69xy>S<o>&x3Cfe57AB;</o>4&lt;Q&x8561;</mmzom7mq9>&lt;&x21d6c2;[</6l><q466uzwais/><bntht2f/>&x7A84;</kqd6h>&lt;&amp;<u/>&x84866E67;&xa6fC;&x6a9CCC4B;</lmk1>A<st1lufkg/>&gt;<jn/>&lt;</ak906t></th8xo>&lt;&xc1;<36txsoso/><pp><6qq69lzd3z><1y55><w></w><qvs/>H&x5AE71194;;&x6c947A82;<sq4hhst/>&lt;f<6/>&x3821A8E4;&amp;</1y55></6qq69lzd3z>4&x2f;<k/></pp><c2g/>&amp;<yubttp3y/>&x1f03727d25;&gt;<njfwl38s/><x><e><t2miweiw><8pc><s0dsg4s><vx6u><4r0qrztsk></4r0qrztsk>&xE9f5;<m/><d403vym/><bf></bf>X&gt;&amp;&xE1adA417;<iz9vezrm3><ofrwov1f>&x17148DbF;<g68ow9r/>E<x><2no></2no>N&amp;<p36a/>?&x5B;z<zcuciin9f/><zfksc12></zfksc12>&gt;</x><5s/><qlcnz/>&gt;&amp;&xEf4d;&x45;</ofrwov1f>5</iz9vezrm3>&xD5;&gt;&xC053715Cd5;&xc509731CFc;%&x08bc5e;</vx6u>&x58;<0b1ode3g/></s0dsg4s><lv/>8&lt;</8pc>&gt;</t2miweiw><a2oz87d><9npr><2z31><70impgzs><f7j><uh9><lzvtrktmvi><35rsra></35rsra>j1&x4B2C0D00;</lzvtrktmvi>&gt;<u/></uh9><4/></f7j><9t41ie><t6yhid37c><9plzb></9plzb>Ts<08by3fqz0m/><nxa3tnjy8o><rp9e></rp9e>&x37ee;y<huz><h9soe16f6><6></6></h9soe16f6>&lt;&x9AA705DB;</huz>QU<1/><2a5></2a5></nxa3tnjy8o>&x1efE0F6D61;&x415854;<ih8q/>&lt;<8v7/>&xd122;<if/><92z8a2iiy/><20a4q67r1/><dm/>&gt;<2d3255/><bp8><ybk6zxkbfc><mjt7><k6wbp>&xcf;</k6wbp>T<2ka91w4></2ka91w4>x&lt;<5/>l&xE603f510;&amp;<ib1rw></ib1rw>&xf94f9590;<13fym1>&x72269B;</13fym1><4anbrdg2>V&x9B8e;u</4anbrdg2><fv1/><19couc/><7iou></7iou><x1r0/><8ucxuulli/>|&amp;<4nh></4nh><8oi/>$&lt;&gt;<5/></mjt7>&lt;</ybk6zxkbfc></bp8><s/>&xe550;&x65;l&xb123;&x374546;n&x1AFBFa7f;-<v71b/><3bggtjge></3bggtjge><ic56w/>&x024c11;</t6yhid37c></9t41ie>&lt;<cq2u96/></70impgzs></2z31><942w0/></9npr>&gt;</a2oz87d></e><vhcy><egvt0g><d5jkxsc><m>&xa92BaA;5&lt;&lt;&lt;<7b2><t6ey><kiec>&x8D616f41;&amp;x!&gt;&x0734;</kiec>&amp;&x58;<rh/>&lt;</t6ey>&x6F46;</7b2><sk2d2l29pn>&gt;<5ryrgjxf8f></5ryrgjxf8f>&x0DFcD36E5F;&lt;<l1q5t><94m1vz/>&x09cF;&amp;&x7B;&x511cBD3124;&xACd7;</l1q5t>&xDB5A706471;p</sk2d2l29pn>&amp;&amp;e&lt;</m>&x553CCe;</d5jkxsc>&x00;</egvt0g>&xABC8a73DdD;</vhcy><p/>xj&amp;<sxh77iw/><4uj/>&amp;[</x>&lt;J<pkbjsc6/><tnt9m/></thxo><bjq01jr><d0d2wjtl0o><06f5iswmk><hiwg>&amp;</hiwg><y84/> kf&lt;<44></44>&lt;<1n4>&x4f937d6D;<0g9lr></0g9lr>P</1n4>&gt;&gt;</06f5iswmk><6khjyyt9/>&amp;<y2yg0n4pvw/><mofm/>=&lt;&amp;&x58B1d9;b&lt;&x14;&amp;;&x208d860C02;<7mgwl0ltx/><tacg9j8yl/>Y&xab;<nsb89hb><cm2568><p1w><hvf>&xb0c3;</hvf>q<muw><h91></h91><8/></muw>&xF839;D&x34;<7yj5/>&amp;L&gt;</p1w>&gt;&amp;<n><pd0hafcbo><xmizbu></xmizbu><8/>&xb977BA;&xA949F3;&lt;<z69/>S</pd0hafcbo><i3d4lg0/></n><f2ge1or5/><50yrnmr/>"</cm2568>&x53F66054;N&lt;V&lt;&x5c639d;<pzcv/>&xD22a7D33;</nsb89hb><xgjtl/><24/>&xF4EFB15665;&lt;<5mp46z/><3oy><y7><g><cx96zpo>&lt;&lt;</cx96zpo>&lt;&gt;</g>&x7e1fA4;</y7>^&amp;&x4787239a0D;</3oy><ypt4/>&gt;[&amp;&xeB2B28;&gt;&amp;&lt;&gt;</d0d2wjtl0o>&amp;&lt;<v92pfefo/>&gt;&gt;&gt;</bjq01jr>&x8eb6c97Ad6;W:</95sibm>&x65;&x103a;&xDa2afB60E3;s<c6lvmnisv></c6lvmnisv>&lt;<8fy/>&xBdA9;&xee15;,&x6Dfe;&lt;7&x82E2;"#;
    let xml = Xml::parse(&mut TokenStream::new(source));

    if xml.is_err() {
        assert_eq!(xml, Err(Error::LoopDetected));
    }
}
