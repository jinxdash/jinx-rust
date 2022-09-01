[                                                                                                                                         /*
[↲    <Program>
[↲    <Program.ast{dk: "None"}>
[↲    <ExpressionStatement{semi}>
[↲    <ArrayLiteral>                                                                                                                      */
	('\x0A', '\x0B', '\x0C', '\x0D', '\x20', '\u{85}', '\u{A0}'),                                                                         /*
	('\x0A',•'\x0B',•'\x0C',•'\x0D',•'\x20',•'\u{85}',•'\u{A0}')    TupleLiteral
	 '\x0A'                                                         Literal{kind: Char}
	         '\x0B'                                                 Literal{kind: Char}
	                 '\x0C'                                         Literal{kind: Char}
	                         '\x0D'                                 Literal{kind: Char}
	                                 '\x20'                         Literal{kind: Char}
	                                         '\u{85}'               Literal{kind: Char}
	                                                   '\u{A0}'     Literal{kind: Char}                                                   */
	('\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}'),                                                                         /*
	('\u{1680}',•'\u{2000}',•'\u{2001}',•'\u{2002}',•'\u{2003}')    TupleLiteral
	 '\u{1680}'                                                     Literal{kind: Char}
	             '\u{2000}'                                         Literal{kind: Char}
	                         '\u{2001}'                             Literal{kind: Char}
	                                     '\u{2002}'                 Literal{kind: Char}
	                                                 '\u{2003}'     Literal{kind: Char}                                                   */
	('\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}'),                                                                         /*
	('\u{2004}',•'\u{2005}',•'\u{2006}',•'\u{2007}',•'\u{2008}')    TupleLiteral
	 '\u{2004}'                                                     Literal{kind: Char}
	             '\u{2005}'                                         Literal{kind: Char}
	                         '\u{2006}'                             Literal{kind: Char}
	                                     '\u{2007}'                 Literal{kind: Char}
	                                                 '\u{2008}'     Literal{kind: Char}                                                   */
	('\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}'),                                                                         /*
	('\u{2009}',•'\u{200A}',•'\u{2028}',•'\u{2029}',•'\u{202F}')    TupleLiteral
	 '\u{2009}'                                                     Literal{kind: Char}
	             '\u{200A}'                                         Literal{kind: Char}
	                         '\u{2028}'                             Literal{kind: Char}
	                                     '\u{2029}'                 Literal{kind: Char}
	                                                 '\u{202F}'     Literal{kind: Char}                                                   */
	('\u{205F}', '\u{3000}'),                                                                                                             /*
	('\u{205F}',•'\u{3000}')    TupleLiteral
	 '\u{205F}'                 Literal{kind: Char}
	             '\u{3000}'     Literal{kind: Char}                                                                                       */
	("", r"\", "\n", "\t", "'", "\r", "\\\n", "\\\n ", "\\\n \u{a0} x","\\\n  \n  x"),                                                    /*
	("",•r"\",•"\n",•"\t",•"'",•"\r",•"\\\n",•"\\\n•",•"\\\n•\u{a0}•x","\\\n••\n••x")    TupleLiteral
	 ""                                                                                  Literal{kind: String}
	     r"\"                                                                            Literal{kind: rString}
	           "\n"                                                                      Literal{kind: String}
	                 "\t"                                                                Literal{kind: String}
	                       "'"                                                           Literal{kind: String}
	                            "\r"                                                     Literal{kind: String}
	                                  "\\\n"                                             Literal{kind: String}
	                                          "\\\n•"                                    Literal{kind: String}
	                                                   "\\\n•\u{a0}•x"                   Literal{kind: String}
	                                                                   "\\\n••\n••x"     Literal{kind: String}                            */
	(r"\u{0}x", r"\u{1F63b}}", r"\v", r"\💩", r"\●", "\\\r"),                                                                             /*
	(r"\u{0}x",•r"\u{1F63b}}",•r"\v",•r"\💩",•r"\●",•"\\\r")    TupleLiteral
	 r"\u{0}x"                                                  Literal{kind: rString}
	            r"\u{1F63b}}"                                   Literal{kind: rString}
	                           r"\v"                            Literal{kind: rString}
	                                  r"\💩"                    Literal{kind: rString}
	                                          r"\●"             Literal{kind: rString}
	                                                 "\\\r"     Literal{kind: String}                                                     */
	(r"\u{FFFFFF}", r"\u{ffffff}", r"\u{ffffff}"),                                                                                        /*
	(r"\u{FFFFFF}",•r"\u{ffffff}",•r"\u{ffffff}")    TupleLiteral
	 r"\u{FFFFFF}"                                   Literal{kind: rString}
	                r"\u{ffffff}"                    Literal{kind: rString}
	                               r"\u{ffffff}"     Literal{kind: rString}                                                               */
	(r"\x", r"\x0", r"\xf", r"\xa", r"\xx", r"\xы", r"\x🦀", r"\xtt", r"\xff", r"\xFF", r"\x80"),                                         /*
	(r"\x",•r"\x0",•r"\xf",•r"\xa",•r"\xx",•r"\xы",•r"\x🦀",•r"\xtt",•r"\xff",•r"\xFF",•r"\x80")    TupleLiteral
	 r"\x"                                                                                          Literal{kind: rString}
	        r"\x0"                                                                                  Literal{kind: rString}
	                r"\xf"                                                                          Literal{kind: rString}
	                        r"\xa"                                                                  Literal{kind: rString}
	                                r"\xx"                                                          Literal{kind: rString}
	                                        r"\xы"                                                  Literal{kind: rString}
	                                                r"\x🦀"                                         Literal{kind: rString}
	                                                         r"\xtt"                                Literal{kind: rString}
	                                                                  r"\xff"                       Literal{kind: rString}
	                                                                           r"\xFF"              Literal{kind: rString}
	                                                                                    r"\x80"     Literal{kind: rString}                */
	(r"\x0ff", r#"\"a"#, r"\na", r"\ra", r"\ta", r"\\a", r"\'a", r"\0a"),                                                                 /*
	(r"\x0ff",•r#"\"a"#,•r"\na",•r"\ra",•r"\ta",•r"\\a",•r"\'a",•r"\0a")    TupleLiteral
	 r"\x0ff"                                                               Literal{kind: rString}
	           r#"\"a"#                                                     Literal{kind: rString}
	                     r"\na"                                             Literal{kind: rString}
	                             r"\ra"                                     Literal{kind: rString}
	                                     r"\ta"                             Literal{kind: rString}
	                                             r"\\a"                     Literal{kind: rString}
	                                                     r"\'a"             Literal{kind: rString}
	                                                             r"\0a"     Literal{kind: rString}                                        */
	(r"\u{DC00}", r"\u{DDDD}", r"\u{DFFF}", r"\u{D800}", r"\u{DAAA}", r"\u{DBFF}"),                                                       /*
	(r"\u{DC00}",•r"\u{DDDD}",•r"\u{DFFF}",•r"\u{D800}",•r"\u{DAAA}",•r"\u{DBFF}")    TupleLiteral
	 r"\u{DC00}"                                                                      Literal{kind: rString}
	              r"\u{DDDD}"                                                         Literal{kind: rString}
	                           r"\u{DFFF}"                                            Literal{kind: rString}
	                                        r"\u{D800}"                               Literal{kind: rString}
	                                                     r"\u{DAAA}"                  Literal{kind: rString}
	                                                                  r"\u{DBFF}"     Literal{kind: rString}                              */
	(r"\u", r"\u[0123]", r"\u{0x}", r"\u{", r"\u{0000", r"\u{}", r"\u{_0000}", r"\u{0000000}"),                                           /*
	(r"\u",•r"\u[0123]",•r"\u{0x}",•r"\u{",•r"\u{0000",•r"\u{}",•r"\u{_0000}",•r"\u{0000000}")    TupleLiteral
	 r"\u"                                                                                        Literal{kind: rString}
	        r"\u[0123]"                                                                           Literal{kind: rString}
	                     r"\u{0x}"                                                                Literal{kind: rString}
	                                r"\u{"                                                        Literal{kind: rString}
	                                        r"\u{0000"                                            Literal{kind: rString}
	                                                    r"\u{}"                                   Literal{kind: rString}
	                                                             r"\u{_0000}"                     Literal{kind: rString}
	                                                                           r"\u{0000000}"     Literal{kind: rString}                  */
	(r"\0", r"\x00", r"\x5a", r"\x5A", r"\x7f", r"\x80", r"\xff", r"\xFF"),                                                               /*
	(r"\0",•r"\x00",•r"\x5a",•r"\x5A",•r"\x7f",•r"\x80",•r"\xff",•r"\xFF")    TupleLiteral
	 r"\0"                                                                    Literal{kind: rString}
	        r"\x00"                                                           Literal{kind: rString}
	                 r"\x5a"                                                  Literal{kind: rString}
	                          r"\x5A"                                         Literal{kind: rString}
	                                   r"\x7f"                                Literal{kind: rString}
	                                            r"\x80"                       Literal{kind: rString}
	                                                     r"\xff"              Literal{kind: rString}
	                                                              r"\xFF"     Literal{kind: rString}                                      */
	("a", r#"\""#, r"\n", r"\r", r"\t", r"\\", r"\'"),                                                                                    /*
	("a",•r#"\""#,•r"\n",•r"\r",•r"\t",•r"\\",•r"\'")    TupleLiteral
	 "a"                                                 Literal{kind: String}
	      r#"\""#                                        Literal{kind: rString}
	               r"\n"                                 Literal{kind: rString}
	                      r"\r"                          Literal{kind: rString}
	                             r"\t"                   Literal{kind: rString}
	                                    r"\\"            Literal{kind: rString}
	                                           r"\'"     Literal{kind: rString}                                                           */
	("a", 'a'),                                                                                                                           /*
	("a",•'a')    TupleLiteral
	 "a"          Literal{kind: String}
	      'a'     Literal{kind: Char}                                                                                                     */
	("ы", 'ы'),                                                                                                                           /*
	("ы",•'ы')    TupleLiteral
	 "ы"          Literal{kind: String}
	      'ы'     Literal{kind: Char}                                                                                                     */
	("🦀", '🦀'),                                                                                                                         /*
	("🦀",•'🦀')    TupleLiteral
	 "🦀"           Literal{kind: String}
	       '🦀'     Literal{kind: Char}                                                                                                   */
	(r#"\""#, '"'),                                                                                                                       /*
	(r#"\""#,•'"')    TupleLiteral
	 r#"\""#          Literal{kind: rString}
	          '"'     Literal{kind: Char}                                                                                                 */
	(r"\n", '\n'),                                                                                                                        /*
	(r"\n",•'\n')    TupleLiteral
	 r"\n"           Literal{kind: rString}
	        '\n'     Literal{kind: Char}                                                                                                  */
	(r"\r", '\r'),                                                                                                                        /*
	(r"\r",•'\r')    TupleLiteral
	 r"\r"           Literal{kind: rString}
	        '\r'     Literal{kind: Char}                                                                                                  */
	(r"\t", '\t'),                                                                                                                        /*
	(r"\t",•'\t')    TupleLiteral
	 r"\t"           Literal{kind: rString}
	        '\t'     Literal{kind: Char}                                                                                                  */
	(r"\\", '\\'),                                                                                                                        /*
	(r"\\",•'\\')    TupleLiteral
	 r"\\"           Literal{kind: rString}
	        '\\'     Literal{kind: Char}                                                                                                  */
	(r"\'", '\''),                                                                                                                        /*
	(r"\'",•'\'')    TupleLiteral
	 r"\'"           Literal{kind: rString}
	        '\''     Literal{kind: Char}                                                                                                  */
	(r"\0", '\0'),                                                                                                                        /*
	(r"\0",•'\0')    TupleLiteral
	 r"\0"           Literal{kind: rString}
	        '\0'     Literal{kind: Char}                                                                                                  */
	(r"\x00", '\0'),                                                                                                                      /*
	(r"\x00",•'\0')    TupleLiteral
	 r"\x00"           Literal{kind: rString}
	          '\0'     Literal{kind: Char}                                                                                                */
	(r"\x5a", 'Z'),                                                                                                                       /*
	(r"\x5a",•'Z')    TupleLiteral
	 r"\x5a"          Literal{kind: rString}
	          'Z'     Literal{kind: Char}                                                                                                 */
	(r"\x5A", 'Z'),                                                                                                                       /*
	(r"\x5A",•'Z')    TupleLiteral
	 r"\x5A"          Literal{kind: rString}
	          'Z'     Literal{kind: Char}                                                                                                 */
	(r"\x7f", 127 as char),                                                                                                               /*
	(r"\x7f",•127•as•char)    TupleLiteral
	 r"\x7f"                  Literal{kind: rString}
	          127•as•char     ExpressionAsTypeCast
	          127             Literal{kind: Integer}                                                                                      */
	(r"\u{0}", '\0'),                                                                                                                     /*
	(r"\u{0}",•'\0')    TupleLiteral
	 r"\u{0}"           Literal{kind: rString}
	           '\0'     Literal{kind: Char}                                                                                               */
	(r"\u{000000}", '\0'),                                                                                                                /*
	(r"\u{000000}",•'\0')    TupleLiteral
	 r"\u{000000}"           Literal{kind: rString}
	                '\0'     Literal{kind: Char}                                                                                          */
	(r"\u{41}", 'A'),                                                                                                                     /*
	(r"\u{41}",•'A')    TupleLiteral
	 r"\u{41}"          Literal{kind: rString}
	            'A'     Literal{kind: Char}                                                                                               */
	(r"\u{0041}", 'A'),                                                                                                                   /*
	(r"\u{0041}",•'A')    TupleLiteral
	 r"\u{0041}"          Literal{kind: rString}
	              'A'     Literal{kind: Char}                                                                                             */
	(r"\u{00_41}", 'A'),                                                                                                                  /*
	(r"\u{00_41}",•'A')    TupleLiteral
	 r"\u{00_41}"          Literal{kind: rString}
	               'A'     Literal{kind: Char}                                                                                            */
	(r"\u{4__1__}", 'A'),                                                                                                                 /*
	(r"\u{4__1__}",•'A')    TupleLiteral
	 r"\u{4__1__}"          Literal{kind: rString}
	                'A'     Literal{kind: Char}                                                                                           */
	(r"\u{1F63b}", '😻'),                                                                                                                 /*
	(r"\u{1F63b}",•'😻')    TupleLiteral
	 r"\u{1F63b}"           Literal{kind: rString}
	               '😻'     Literal{kind: Char}                                                                                           */
	(b"a\n\r\t\\\'\"\0\xF0", br###"a"##b"###, b"a\"##b"),                                                                                 /*
	(b"a\n\r\t\\\'\"\0\xF0",•br###"a"##b"###,•b"a\"##b")    TupleLiteral
	 b"a\n\r\t\\\'\"\0\xF0"                                 Literal{kind: bString}
	                         br###"a"##b"###                Literal{kind: brString}
	                                          b"a\"##b"     Literal{kind: bString}                                                        */
	(b"a\xF0\t",b'\xF0',br"a\n", b'a', b'\n', b'\r', b'\t', b'\\', b'\'', b'\"', b'\0', b'\xF0'),                                         /*
	(b"a\xF0\t",b'\xF0',br"a\n",•b'a',•b'\n',•b'\r',•b'\t',•b'\\',•b'\'',•b'\"',•b'\0',•b'\xF0')    TupleLiteral
	 b"a\xF0\t"                                                                                     Literal{kind: bString}
	            b'\xF0'                                                                             Literal{kind: bChar}
	                    br"a\n"                                                                     Literal{kind: brString}
	                             b'a'                                                               Literal{kind: bChar}
	                                   b'\n'                                                        Literal{kind: bChar}
	                                          b'\r'                                                 Literal{kind: bChar}
	                                                 b'\t'                                          Literal{kind: bChar}
	                                                        b'\\'                                   Literal{kind: bChar}
	                                                               b'\''                            Literal{kind: bChar}
	                                                                      b'\"'                     Literal{kind: bChar}
	                                                                             b'\0'              Literal{kind: bChar}
	                                                                                    b'\xF0'     Literal{kind: bChar}                  */
	(&1u16, &42i32, !0 as u32, !0 as u64),                                                                                                /*
	(&1u16,•&42i32,•!0•as•u32,•!0•as•u64)    TupleLiteral
	 &1u16                                   ReferenceExpression{!mut}
	  1u16                                   Literal{kind: Integer}
	   u16                                   Identifier
	        &42i32                           ReferenceExpression{!mut}
	         42i32                           Literal{kind: Integer}
	           i32                           Identifier
	                !0•as•u32                ExpressionAsTypeCast
	                !0                       NotExpression
	                 0                       Literal{kind: Integer}
	                           !0•as•u64     ExpressionAsTypeCast
	                           !0            NotExpression
	                            0            Literal{kind: Integer}                                                                       */
	(4294967295, 0xffffffff, 0xffffffffffffffff, 18446744073709551615),                                                                   /*
	(4294967295,•0xffffffff,•0xffffffffffffffff,•18446744073709551615)    TupleLiteral
	 4294967295                                                           Literal{kind: Integer}
	             0xffffffff                                               Literal{kind: Hex}
	                         0xffffffffffffffff                           Literal{kind: Hex}
	                                             18446744073709551615     Literal{kind: Integer}                                          */
	(-2147483648i32).wrapping_sub(1), 2147483647,                                                                                         /*
	(-2147483648i32).wrapping_sub(1)                CallExpression
	 -2147483648i32                                 MinusExpression
	  2147483648i32                                 Literal{kind: Integer}
	            i32                                 Identifier
	                             (1)                CallExpression.arguments{dk: "()"}
	                              1                 Literal{kind: Integer}
	                                  2147483647    Literal{kind: Integer}                                                                */
	(-3.40282356e+38_f32, f32::MIN, 3.40282356e+38_f32, f32::MAX),                                                                        /*
	(-3.40282356e+38_f32,•f32::MIN,•3.40282356e+38_f32,•f32::MAX)    TupleLiteral
	 -3.40282356e+38_f32                                             MinusExpression
	  3.40282356e+38_f32                                             Literal{kind: Float}
	                 f32                                             Identifier
	                      f32::MIN                                   ExpressionPath
	                                3.40282356e+38_f32               Literal{kind: Float}
	                                               f32               Identifier
	                                                    f32::MAX     ExpressionPath                                                       */
	(-1.7976931348623158e+308_f64, f64::MIN, 1.7976931348623158e+308_f64, f64::MAX),                                                      /*
	(-1.7976931348623158e+308_f64,•f64::MIN,•1.7976931348623158e+308_f64,•f64::MAX)    TupleLiteral
	 -1.7976931348623158e+308_f64                                                      MinusExpression
	  1.7976931348623158e+308_f64                                                      Literal{kind: Float}
	                          f64                                                      Identifier
	                               f64::MIN                                            ExpressionPath
	                                         1.7976931348623158e+308_f64               Literal{kind: Float}
	                                                                 f64               Identifier
	                                                                      f64::MAX     ExpressionPath                                     */
	(!0xf0_isize & 0xff, 0xf0_isize | 0xf, 0xf_isize << 4, 0xf0_isize >> 4),                                                              /*
	(!0xf0_isize•&•0xff,•0xf0_isize•|•0xf,•0xf_isize•<<•4,•0xf0_isize•>>•4)    TupleLiteral
	 !0xf0_isize•&•0xff                                                        OperationExpression{tk: "&"}
	 !0xf0_isize                                                               NotExpression
	  0xf0_isize                                                               Literal{kind: Hex}
	       isize                                                               Identifier
	               0xff                                                        Literal{kind: Hex}
	                     0xf0_isize•|•0xf                                      OperationExpression{tk: "|"}
	                     0xf0_isize                                            Literal{kind: Hex}
	                          isize                                            Identifier
	                                  0xf                                      Literal{kind: Hex}
	                                       0xf_isize•<<•4                      OperationExpression{tk: "<<"}
	                                       0xf_isize                           Literal{kind: Hex}
	                                           isize                           Identifier
	                                                    4                      Literal{kind: Integer}
	                                                       0xf0_isize•>>•4     OperationExpression{tk: ">>"}
	                                                       0xf0_isize          Literal{kind: Hex}
	                                                            isize          Identifier
	                                                                     4     Literal{kind: Integer}                                     */
	(0b1010_1010_isize | 0b0101_0101, -1000isize as usize >> 3_usize, 2305843009213693827_usize),                                         /*
	(0b1010_1010_isize•|•0b0101_0101,•-1000isize•as•usize•>>•3_usize,•2305843009213693827_usize)    TupleLiteral
	 0b1010_1010_isize•|•0b0101_0101                                                                OperationExpression{tk: "|"}
	 0b1010_1010_isize                                                                              Literal{kind: Binary}
	             isize                                                                              Identifier
	                     0b0101_0101                                                                Literal{kind: Binary}
	                                  -1000isize•as•usize•>>•3_usize                                OperationExpression{tk: ">>"}
	                                  -1000isize•as•usize                                           ExpressionAsTypeCast
	                                  -1000isize                                                    MinusExpression
	                                   1000isize                                                    Literal{kind: Integer}
	                                       isize                                                    Identifier
	                                                         3_usize                                Literal{kind: Integer}
	                                                           usize                                Identifier
	                                                                  2305843009213693827_usize     Literal{kind: Integer}
	                                                                                      usize     Identifier                            */
	(-16 >> 2, a.0-1, a.0.1..2, 0.b0),                                                                                                    /*
	(-16•>>•2,•a.0-1,•a.0.1..2,•0.b0)    TupleLiteral
	 -16•>>•2                            OperationExpression{tk: ">>"}
	 -16                                 MinusExpression
	  16                                 Literal{kind: Integer}
	        2                            Literal{kind: Integer}
	           a.0-1                     OperationExpression{tk: "-"}
	           a.0                       MemberExpression{!computed}
	             0                       Index
	               1                     Literal{kind: Integer}
	                  a.0.1..2           RangeLiteral{!last}
	                  a.0.1              MemberExpression{!computed}
	                  a.0                MemberExpression{!computed}
	                    0                Index
	                      1              Index
	                         2           Literal{kind: Integer}
	                            0.b0     MemberExpression{!computed}
	                            0        Literal{kind: Integer}                                                                           */
	('\u{10__FFFF}', "\u{10_F0FF__}foo\u{1_0_0_0__}"),                                                                                    /*
	('\u{10__FFFF}',•"\u{10_F0FF__}foo\u{1_0_0_0__}")    TupleLiteral
	 '\u{10__FFFF}'                                      Literal{kind: Char}
	                 "\u{10_F0FF__}foo\u{1_0_0_0__}"     Literal{kind: String}                                                            */
	(0, 1, 0.1, 1.1, 1., 1.00500, 1.0, 1.5, 1.50, 0.00500, 0.0, 0.0000),                                                                  /*
	(0,•1,•0.1,•1.1,•1.,•1.00500,•1.0,•1.5,•1.50,•0.00500,•0.0,•0.0000)    TupleLiteral
	 0                                                                     Literal{kind: Integer}
	    1                                                                  Literal{kind: Integer}
	       0.1                                                             Literal{kind: Float}
	            1.1                                                        Literal{kind: Float}
	                 1.                                                    Literal{kind: Float}
	                     1.00500                                           Literal{kind: Float}
	                              1.0                                      Literal{kind: Float}
	                                   1.5                                 Literal{kind: Float}
	                                        1.50                           Literal{kind: Float}
	                                              0.00500                  Literal{kind: Float}
	                                                       0.0             Literal{kind: Float}
	                                                            0.0000     Literal{kind: Float}                                           */
	(0b1, 0B1, 0o1, 0O1, 0x1, 0X1),                                                                                                       /*
	(0b1,•0B1,•0o1,•0O1,•0x1,•0X1)    TupleLiteral
	 0b1                              Literal{kind: Binary}
	      0B1                         Literal{kind: Integer}
	       B1                         Identifier
	           0o1                    Literal{kind: Octal}
	                0O1               Literal{kind: Integer}
	                 O1               Identifier
	                     0x1          Literal{kind: Hex}
	                          0X1     Literal{kind: Integer}
	                           X1     Identifier                                                                                          */
	(0x123abcdef456ABCDEF, 0X123abcdef456ABCDEF, 0xdeadbeef),                                                                             /*
	(0x123abcdef456ABCDEF,•0X123abcdef456ABCDEF,•0xdeadbeef)    TupleLiteral
	 0x123abcdef456ABCDEF                                       Literal{kind: Hex}
	                       0X123abcdef456ABCDEF                 Literal{kind: Integer}
	                        X123abcdef456ABCDEF                 Identifier
	                                             0xdeadbeef     Literal{kind: Hex}                                                        */
	(0b111000, 0b000111, 0B111000, 0B000111, 0o111000, 0o000111, 0O111000, 0O000111, 0x111000, 0x000111, 0X111000, 0X000111),             /*
	(0b111000,•0b000111,•0B111000,•0B000111,•0o111000,•0o000111,•0O111000,•0O000111,•0x111000,•0x000111,•0X111000,•0X000111)    TupleLiteral
	 0b111000                                                                                                                   Literal{kind: Binary}
	           0b000111                                                                                                         Literal{kind: Binary}
	                     0B111000                                                                                               Literal{kind: Integer}
	                      B111000                                                                                               Identifier
	                               0B000111                                                                                     Literal{kind: Integer}
	                                B000111                                                                                     Identifier
	                                         0o111000                                                                           Literal{kind: Octal}
	                                                   0o000111                                                                 Literal{kind: Octal}
	                                                             0O111000                                                       Literal{kind: Integer}
	                                                              O111000                                                       Identifier
	                                                                       0O000111                                             Literal{kind: Integer}
	                                                                        O000111                                             Identifier
	                                                                                 0x111000                                   Literal{kind: Hex}
	                                                                                           0x000111                         Literal{kind: Hex}
	                                                                                                     0X111000               Literal{kind: Integer}
	                                                                                                      X111000               Identifier
	                                                                                                               0X000111     Literal{kind: Integer}
	                                                                                                                X000111     Identifier*/
	(1e1, 1e+1, 1e-1, 1.e1, 0.1e1, 1.1e1, 1.1e0010, 0.1e+0010, 0.1e-0010, 1E1, 1E+1, 1E-1, 1.E1, 0.1E1, 1.1E1, 1.1E0010, 0.1E+0010, 0.1E-0010),/*
	(1e1,•1e+1,•1e-1,•1.e1,•0.1e1,•1.1e1,•1.1e0010,•0.1e+0010,•0.1e-0010,•1E1,•1E+1,•1E-1,•1.E1,•0.1E1,•1.1E1,•1.1E0010,•0.1E+0010,•0.1E-0010)    TupleLiteral
	 1e1                                                                                                                                          Literal{kind: Float}
	      1e+1                                                                                                                                    Literal{kind: Float}
	            1e-1                                                                                                                              Literal{kind: Float}
	                  1.e1                                                                                                                        MemberExpression{!computed}
	                  1                                                                                                                           Literal{kind: Integer}
	                        0.1e1                                                                                                                 Literal{kind: Float}
	                               1.1e1                                                                                                          Literal{kind: Float}
	                                      1.1e0010                                                                                                Literal{kind: Float}
	                                                0.1e+0010                                                                                     Literal{kind: Float}
	                                                           0.1e-0010                                                                          Literal{kind: Float}
	                                                                      1E1                                                                     Literal{kind: Float}
	                                                                           1E+1                                                               Literal{kind: Float}
	                                                                                 1E-1                                                         Literal{kind: Float}
	                                                                                       1.E1                                                   MemberExpression{!computed}
	                                                                                       1                                                      Literal{kind: Integer}
	                                                                                             0.1E1                                            Literal{kind: Float}
	                                                                                                    1.1E1                                     Literal{kind: Float}
	                                                                                                           1.1E0010                           Literal{kind: Float}
	                                                                                                                     0.1E+0010                Literal{kind: Float}
	                                                                                                                                0.1E-0010     Literal{kind: Float}*/
	(0.5e0, 0.5e00, 0.5e+0, 0.5e+00, 0.5e-0, 0.5e-00),                                                                                    /*
	(0.5e0,•0.5e00,•0.5e+0,•0.5e+00,•0.5e-0,•0.5e-00)    TupleLiteral
	 0.5e0                                               Literal{kind: Float}
	        0.5e00                                       Literal{kind: Float}
	                0.5e+0                               Literal{kind: Float}
	                        0.5e+00                      Literal{kind: Float}
	                                 0.5e-0              Literal{kind: Float}
	                                         0.5e-00     Literal{kind: Float}                                                             */
	(500600.001230045000, 1.00500e60, 1.0e60, 0.00500e60, 0.0e60, 0.0000e60, 0.0e60, 0.e60, 0e60, 500600.001230045000e60),                /*
	(500600.001230045000,•1.00500e60,•1.0e60,•0.00500e60,•0.0e60,•0.0000e60,•0.0e60,•0.e60,•0e60,•500600.001230045000e60)    TupleLiteral
	 500600.001230045000                                                                                                     Literal{kind: Float}
	                      1.00500e60                                                                                         Literal{kind: Float}
	                                  1.0e60                                                                                 Literal{kind: Float}
	                                          0.00500e60                                                                     Literal{kind: Float}
	                                                      0.0e60                                                             Literal{kind: Float}
	                                                              0.0000e60                                                  Literal{kind: Float}
	                                                                         0.0e60                                          Literal{kind: Float}
	                                                                                 0.e60                                   MemberExpression{!computed}
	                                                                                 0                                       Literal{kind: Integer}
	                                                                                        0e60                             Literal{kind: Float}
	                                                                                              500600.001230045000e60     Literal{kind: Float}*/
	(10, 9700, 10e100, 1_1, 1_1.1_1, 0o1_1, 0o0_11, 1.1_0_1e1, 1.1_0_1E1, 0.1_1, 0x1_1, 0xa_1, 0xA_1, 0b01_1, 0b0_1_1),                   /*
	(10,•9700,•10e100,•1_1,•1_1.1_1,•0o1_1,•0o0_11,•1.1_0_1e1,•1.1_0_1E1,•0.1_1,•0x1_1,•0xa_1,•0xA_1,•0b01_1,•0b0_1_1)    TupleLiteral
	 10                                                                                                                   Literal{kind: Integer}
	     9700                                                                                                             Literal{kind: Integer}
	           10e100                                                                                                     Literal{kind: Float}
	                   1_1                                                                                                Literal{kind: Integer}
	                        1_1.1_1                                                                                       Literal{kind: Float}
	                                 0o1_1                                                                                Literal{kind: Octal}
	                                        0o0_11                                                                        Literal{kind: Octal}
	                                                1.1_0_1e1                                                             Literal{kind: Float}
	                                                           1.1_0_1E1                                                  Literal{kind: Float}
	                                                                      0.1_1                                           Literal{kind: Float}
	                                                                             0x1_1                                    Literal{kind: Hex}
	                                                                                    0xa_1                             Literal{kind: Hex}
	                                                                                           0xA_1                      Literal{kind: Hex}
	                                                                                                  0b01_1              Literal{kind: Binary}
	                                                                                                          0b0_1_1     Literal{kind: Binary}*/
	(100).toString(),                                                                                                                     /*
	(100).toString()    CallExpression
	 100                Literal{kind: Integer}
	              ()    CallExpression.arguments{dk: "()"}                                                                                */
	f!("\0="),                                                                                                                            /*
	f!("\0=")    MacroInvocation
	  ("\0=")    MacroInvocation.segments{dk: "()"}
	   "\0="     Literal{kind: String}                                                                                                    */
	2.f64,                                                                                                                                /*
	2.f64    MemberExpression{!computed}
	2        Literal{kind: Integer}                                                                                                       */
	2.f64(),                                                                                                                              /*
	2.f64()    CallExpression
	2          Literal{kind: Integer}
	     ()    CallExpression.arguments{dk: "()"}                                                                                         */
];                                                                                                                                        /*
]     </ArrayLiteral>
];    </ExpressionStatement>                                                                                                              */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
	let n: f64 = 1234567890123456789012345678901234567890e-340;                                                                           /*
	let•n:•f64•=•1234567890123456789012345678901234567890e-340;    LetVariableDeclaration
	             1234567890123456789012345678901234567890e-340     Literal{kind: Float}                                                   */
	let n: f64 = 0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333;/*
	let•n:•f64•=•0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333;    LetVariableDeclaration
	             0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333     Literal{kind: Float}*/
    let s = "string                                                                                                                      "/*
    let•s•=•"string↲    <LetVariableDeclaration>
            "string↲    <Literal{kind: String}>                                                                                          */"
literal";                                                                                                                                 /*
literal"     </Literal>
literal";    </LetVariableDeclaration>                                                                                                    */
    let s = "literal with \                                                                                                              "/*
    let•s•=•"literal•with•\↲    <LetVariableDeclaration>
            "literal•with•\↲    <Literal{kind: String}>                                                                                  */"
             escaped newline";                                                                                                            /*
•••••••••••••escaped•newline"     </Literal>
•••••••••••••escaped•newline";    </LetVariableDeclaration>                                                                               */
    let s = r"string                                                                                                                     "/*
    let•s•=•r"string↲    <LetVariableDeclaration>
            r"string↲    <Literal{kind: rString}>                                                                                       */r"
literal";                                                                                                                                 /*
literal"     </Literal>
literal";    </LetVariableDeclaration>                                                                                                    */
    let s = br"byte string                                                                                                               "/*
    let•s•=•br"byte•string↲    <LetVariableDeclaration>
            br"byte•string↲    <Literal{kind: brString}>                                                                               */br"
literal";                                                                                                                                 /*
literal"     </Literal>
literal";    </LetVariableDeclaration>                                                                                                    */
	let s = "foo\r\nbar\n\nbaz\n";                                                                                                        /*
	let•s•=•"foo\r\nbar\n\nbaz\n";    LetVariableDeclaration
	        "foo\r\nbar\n\nbaz\n"     Literal{kind: String}                                                                               */
	let v = !((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//
/*	let•v•=•!((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//↲    <LetVariableDeclaration>
	        !((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})       NotExpression
	          (|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];}        ComparisonExpression{tk: "=="}
	          (|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})                         CallExpression
	           |(..):(_,_),__@_|__                                                ClosureFunctionExpression
	           |(..):(_,_),__@_|                                                  ClosureFunctionExpression.parameters{dk: "||"}
	            (..):(_,_)                                                        ClosureFunctionParameterDeclaration
	            (..)                                                              TuplePattern
	             ..                                                               RestPattern
	                 (_,_)                                                        TypeTuple
	                  _                                                           TypeInferred
	                    _                                                         TypeInferred
	                       __@_                                                   ClosureFunctionParameterDeclaration, PatternVariableDeclaration{!ref, !mut}
	                          _                                                   WildcardPattern
	                               ((&*"\\",'🤔')/**/,{})                         CallExpression.arguments{dk: "()"}
	                                (&*"\\",'🤔')                                 TupleLiteral
	                                 &*"\\"                                       ReferenceExpression{!mut}
	                                  *"\\"                                       DereferenceExpression
	                                   "\\"                                       Literal{kind: String}
	                                        '🤔'                                  Literal{kind: Char}
	                                             /**/                             Comment{!line}
	                                                  {}                          BlockExpression
	                                                       {&[..=..][..];}        BlockExpression
	                                                        &[..=..][..];         ExpressionStatement{semi}
	                                                        &[..=..][..]          ReferenceExpression{!mut}
	                                                         [..=..][..]          MemberExpression{computed}
	                                                         [..=..]              ArrayLiteral
	                                                          ..=..               RangeLiteral{last}
	                                                             ..               RangeLiteral{!last}
	                                                                 ..           RangeLiteral{!last}                                     */
	                                                                       //     Comment{line}
    ;                                                                                                                                     /*
••••;    </LetVariableDeclaration>                                                                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 4
// Parsed Nodes: 474
// state_rollbacks: 2
// Total '.charCodeAt()' calls: 4113 (17% re-reads)
// Unnecessary 'skip_whitespace()' calls: 176
// source: "../../samples/expressions/literal.rs"