['\x0A', '\x0B', '\x0C', '\x0D', '\x20', '\u{85}', '\u{A0}',                                                                              /*
['\x0A',•'\x0B',•'\x0C',•'\x0D',•'\x20',•'\u{85}',•'\u{A0}',↲    <ExpressionStatement>, <ArrayLiteral>
 '\x0A'                                                          Literal
         '\x0B'                                                  Literal
                 '\x0C'                                          Literal
                         '\x0D'                                  Literal
                                 '\x20'                          Literal
                                         '\u{85}'                Literal
                                                   '\u{A0}'      Literal                                                                  */
 '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',                                                                              /*
 '\u{1680}'                                                     Literal
             '\u{2000}'                                         Literal
                         '\u{2001}'                             Literal
                                     '\u{2002}'                 Literal
                                                 '\u{2003}'     Literal                                                                   */
 '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',                                                                              /*
 '\u{2004}'                                                     Literal
             '\u{2005}'                                         Literal
                         '\u{2006}'                             Literal
                                     '\u{2007}'                 Literal
                                                 '\u{2008}'     Literal                                                                   */
 '\u{2009}', '\u{200A}', '\u{2028}', '\u{2029}', '\u{202F}',                                                                              /*
 '\u{2009}'                                                     Literal
             '\u{200A}'                                         Literal
                         '\u{2028}'                             Literal
                                     '\u{2029}'                 Literal
                                                 '\u{202F}'     Literal                                                                   */
 '\u{205F}', '\u{3000}'];                                                                                                                 /*
•'\u{205F}',•'\u{3000}'];    </ExpressionStatement>
•'\u{205F}',•'\u{3000}']     </ArrayLiteral>
 '\u{205F}'                  Literal
             '\u{3000}'      Literal                                                                                                      */
[                                                                                                                                         /*
[╚↲    <ExpressionStatement>, <ArrayLiteral>                                                                                              */
	"", r"\", "\n", "\t", "'", "\r", "\\\n", "\\\n ", "\\\n \u{a0} x","\\\n  \n  x",                                                      /*
    ""                                                                                 Literal
        r"\"                                                                           Literal
              "\n"                                                                     Literal
                    "\t"                                                               Literal
                          "'"                                                          Literal
                               "\r"                                                    Literal
                                     "\\\n"                                            Literal
                                             "\\\n•"                                   Literal
                                                      "\\\n•\u{a0}•x"                  Literal
                                                                      "\\\n••\n••x"    Literal                                            */
	"spam", r"\x0ff", r#"\"a"#, r"\na", r"\ra", r"\ta", r"\\a", r"\'a", r"\0a",                                                           /*
    "spam"                                                                        Literal
            r"\x0ff"                                                              Literal
                      r#"\"a"#                                                    Literal
                                r"\na"                                            Literal
                                        r"\ra"                                    Literal
                                                r"\ta"                            Literal
                                                        r"\\a"                    Literal
                                                                r"\'a"            Literal
                                                                        r"\0a"    Literal                                                 */
	r"\u{0}x", r"\u{1F63b}}", r"\v", r"\💩", r"\●", "\\\r",                                                                               /*
    r"\u{0}x"                                                 Literal
               r"\u{1F63b}}"                                  Literal
                              r"\v"                           Literal
                                     r"\💩"                   Literal
                                             r"\●"            Literal
                                                    "\\\r"    Literal                                                                     */
	r"\x", r"\x0", r"\xf", r"\xa", r"\xx", r"\xы", r"\x🦀", r"\xtt", r"\xff", r"\xFF", r"\x80",                                           /*
    r"\x"                                                                                         Literal
           r"\x0"                                                                                 Literal
                   r"\xf"                                                                         Literal
                           r"\xa"                                                                 Literal
                                   r"\xx"                                                         Literal
                                           r"\xы"                                                 Literal
                                                   r"\x🦀"                                        Literal
                                                            r"\xtt"                               Literal
                                                                     r"\xff"                      Literal
                                                                              r"\xFF"             Literal
                                                                                       r"\x80"    Literal                                 */
	r"\u", r"\u[0123]", r"\u{0x}", r"\u{", r"\u{0000", r"\u{}", r"\u{_0000}", r"\u{0000000}",                                             /*
    r"\u"                                                                                       Literal
           r"\u[0123]"                                                                          Literal
                        r"\u{0x}"                                                               Literal
                                   r"\u{"                                                       Literal
                                           r"\u{0000"                                           Literal
                                                       r"\u{}"                                  Literal
                                                                r"\u{_0000}"                    Literal
                                                                              r"\u{0000000}"    Literal                                   */
	r"\u{FFFFFF}", r"\u{ffffff}", r"\u{ffffff}",                                                                                          /*
    r"\u{FFFFFF}"                                  Literal
                   r"\u{ffffff}"                   Literal
                                  r"\u{ffffff}"    Literal                                                                                */
	r"\u{DC00}", r"\u{DDDD}", r"\u{DFFF}", r"\u{D800}", r"\u{DAAA}", r"\u{DBFF}",                                                         /*
    r"\u{DC00}"                                                                     Literal
                 r"\u{DDDD}"                                                        Literal
                              r"\u{DFFF}"                                           Literal
                                           r"\u{D800}"                              Literal
                                                        r"\u{DAAA}"                 Literal
                                                                     r"\u{DBFF}"    Literal                                               */
	"a", r#"\""#, r"\n", r"\r", r"\t", r"\\", r"\'",                                                                                      /*
    "a"                                                Literal
         r#"\""#                                       Literal
                  r"\n"                                Literal
                         r"\r"                         Literal
                                r"\t"                  Literal
                                       r"\\"           Literal
                                              r"\'"    Literal                                                                            */
	r"\0", r"\x00", r"\x5a", r"\x5A", r"\x7f", r"\x80", r"\xff", r"\xFF",                                                                 /*
    r"\0"                                                                   Literal
           r"\x00"                                                          Literal
                    r"\x5a"                                                 Literal
                             r"\x5A"                                        Literal
                                      r"\x7f"                               Literal
                                               r"\x80"                      Literal
                                                        r"\xff"             Literal
                                                                 r"\xFF"    Literal                                                       */
];                                                                                                                                        /*
];    </ExpressionStatement>
]     </ArrayLiteral>                                                                                                                     */
[                                                                                                                                         /*
[↲    <ExpressionStatement>, <ArrayLiteral>                                                                                               */
	("a", 'a'),                                                                                                                           /*
    ("a",•'a')    TupleLiteral
     "a"          Literal
          'a'     Literal                                                                                                                 */
	("ы", 'ы'),                                                                                                                           /*
    ("ы",•'ы')    TupleLiteral
     "ы"          Literal
          'ы'     Literal                                                                                                                 */
	("🦀", '🦀'),                                                                                                                         /*
    ("🦀",•'🦀')    TupleLiteral
     "🦀"           Literal
           '🦀'     Literal                                                                                                               */
	(r#"\""#, '"'),                                                                                                                       /*
    (r#"\""#,•'"')    TupleLiteral
     r#"\""#          Literal
              '"'     Literal                                                                                                             */
	(r"\n", '\n'),                                                                                                                        /*
    (r"\n",•'\n')    TupleLiteral
     r"\n"           Literal
            '\n'     Literal                                                                                                              */
	(r"\r", '\r'),                                                                                                                        /*
    (r"\r",•'\r')    TupleLiteral
     r"\r"           Literal
            '\r'     Literal                                                                                                              */
	(r"\t", '\t'),                                                                                                                        /*
    (r"\t",•'\t')    TupleLiteral
     r"\t"           Literal
            '\t'     Literal                                                                                                              */
	(r"\\", '\\'),                                                                                                                        /*
    (r"\\",•'\\')    TupleLiteral
     r"\\"           Literal
            '\\'     Literal                                                                                                              */
	(r"\'", '\''),                                                                                                                        /*
    (r"\'",•'\'')    TupleLiteral
     r"\'"           Literal
            '\''     Literal                                                                                                              */
	(r"\0", '\0'),                                                                                                                        /*
    (r"\0",•'\0')    TupleLiteral
     r"\0"           Literal
            '\0'     Literal                                                                                                              */
	(r"\x00", '\0'),                                                                                                                      /*
    (r"\x00",•'\0')    TupleLiteral
     r"\x00"           Literal
              '\0'     Literal                                                                                                            */
	(r"\x5a", 'Z'),                                                                                                                       /*
    (r"\x5a",•'Z')    TupleLiteral
     r"\x5a"          Literal
              'Z'     Literal                                                                                                             */
	(r"\x5A", 'Z'),                                                                                                                       /*
    (r"\x5A",•'Z')    TupleLiteral
     r"\x5A"          Literal
              'Z'     Literal                                                                                                             */
	(r"\x7f", 127 as char),                                                                                                               /*
    (r"\x7f",•127•as•char)    TupleLiteral
     r"\x7f"                  Literal
              127•as•char     ExpressionAsTypeCast
              127             Literal                                                                                                     */
	(r"\u{0}", '\0'),                                                                                                                     /*
    (r"\u{0}",•'\0')    TupleLiteral
     r"\u{0}"           Literal
               '\0'     Literal                                                                                                           */
	(r"\u{000000}", '\0'),                                                                                                                /*
    (r"\u{000000}",•'\0')    TupleLiteral
     r"\u{000000}"           Literal
                    '\0'     Literal                                                                                                      */
	(r"\u{41}", 'A'),                                                                                                                     /*
    (r"\u{41}",•'A')    TupleLiteral
     r"\u{41}"          Literal
                'A'     Literal                                                                                                           */
	(r"\u{0041}", 'A'),                                                                                                                   /*
    (r"\u{0041}",•'A')    TupleLiteral
     r"\u{0041}"          Literal
                  'A'     Literal                                                                                                         */
	(r"\u{00_41}", 'A'),                                                                                                                  /*
    (r"\u{00_41}",•'A')    TupleLiteral
     r"\u{00_41}"          Literal
                   'A'     Literal                                                                                                        */
	(r"\u{4__1__}", 'A'),                                                                                                                 /*
    (r"\u{4__1__}",•'A')    TupleLiteral
     r"\u{4__1__}"          Literal
                    'A'     Literal                                                                                                       */
	(r"\u{1F63b}", '😻'),                                                                                                                 /*
    (r"\u{1F63b}",•'😻')    TupleLiteral
     r"\u{1F63b}"           Literal
                   '😻'     Literal                                                                                                       */
	b"a\xF0\t",b'\xF0',br"a\n", b'a',                                                                                                     /*
    b"a\xF0\t"                          Literal
               b'\xF0'                  Literal
                       br"a\n"          Literal
                                b'a'    Literal                                                                                           */
	b'\n', b'\r', b'\t', b'\\', b'\'', b'\"', b'\0', b'\xF0',                                                                             /*
    b'\n'                                                       Literal
           b'\r'                                                Literal
                  b'\t'                                         Literal
                         b'\\'                                  Literal
                                b'\''                           Literal
                                       b'\"'                    Literal
                                              b'\0'             Literal
                                                     b'\xF0'    Literal                                                                   */
	b"a\n\r\t\\\'\"\0\xF0",                                                                                                               /*
    b"a\n\r\t\\\'\"\0\xF0"    Literal                                                                                                     */
	br###"a"##b"###,                                                                                                                      /*
    br###"a"##b"###    Literal                                                                                                            */
	b"a\"##b",                                                                                                                            /*
    b"a\"##b"    Literal                                                                                                                  */
	&1u16,                                                                                                                                /*
    &1u16    ReferenceExpression
     1u16    Literal
      u16    Identifier                                                                                                                   */
	0xffffffff, (!0 as u32),                                                                                                              /*
    0xffffffff                Literal
                 !0•as•u32    ExpressionAsTypeCast
                 !0           NotExpression
                  0           Literal                                                                                                     */
	4294967295, (!0 as u32),                                                                                                              /*
    4294967295                Literal
                 !0•as•u32    ExpressionAsTypeCast
                 !0           NotExpression
                  0           Literal                                                                                                     */
	0xffffffffffffffff, (!0 as u64),                                                                                                      /*
    0xffffffffffffffff                Literal
                         !0•as•u64    ExpressionAsTypeCast
                         !0           NotExpression
                          0           Literal                                                                                             */
	18446744073709551615, (!0 as u64),                                                                                                    /*
    18446744073709551615                Literal
                           !0•as•u64    ExpressionAsTypeCast
                           !0           NotExpression
                            0           Literal                                                                                           */
	(-2147483648i32).wrapping_sub(1), 2147483647,                                                                                         /*
    (-2147483648i32).wrapping_sub(1)                CallExpression
     -2147483648i32                                 MinusExpression
      2147483648i32                                 Literal
                i32                                 Identifier
                                  1                 Literal
                                      2147483647    Literal                                                                               */
	-3.40282356e+38_f32, f32::MIN,                                                                                                        /*
    -3.40282356e+38_f32              MinusExpression
     3.40282356e+38_f32              Literal
                    f32              Identifier
                         f32::MIN    ExpressionPath                                                                                       */
	3.40282356e+38_f32, f32::MAX,                                                                                                         /*
    3.40282356e+38_f32              Literal
                   f32              Identifier
                        f32::MAX    ExpressionPath                                                                                        */
	-1.7976931348623158e+308_f64, f64::MIN,                                                                                               /*
    -1.7976931348623158e+308_f64              MinusExpression
     1.7976931348623158e+308_f64              Literal
                             f64              Identifier
                                  f64::MIN    ExpressionPath                                                                              */
	1.7976931348623158e+308_f64, f64::MAX,                                                                                                /*
    1.7976931348623158e+308_f64              Literal
                            f64              Identifier
                                 f64::MAX    ExpressionPath                                                                               */
	!0xf0_isize & 0xff,                                                                                                                   /*
    !0xf0_isize•&•0xff    OperationExpression
    !0xf0_isize           NotExpression
     0xf0_isize           Literal
          isize           Identifier
                  0xff    Literal                                                                                                         */
	0xf0_isize | 0xf,                                                                                                                     /*
    0xf0_isize•|•0xf    OperationExpression
    0xf0_isize          Literal
         isize          Identifier
                 0xf    Literal                                                                                                           */
	0xf_isize << 4,                                                                                                                       /*
    0xf_isize•<<•4    OperationExpression
    0xf_isize         Literal
        isize         Identifier
                 4    Literal                                                                                                             */
	0xf0_isize >> 4,                                                                                                                      /*
    0xf0_isize•>>•4    OperationExpression
    0xf0_isize         Literal
         isize         Identifier
                  4    Literal                                                                                                            */
	-16 >> 2,                                                                                                                             /*
    -16•>>•2    OperationExpression
    -16         MinusExpression
     16         Literal
           2    Literal                                                                                                                   */
	0b1010_1010_isize | 0b0101_0101,                                                                                                      /*
    0b1010_1010_isize•|•0b0101_0101    OperationExpression
    0b1010_1010_isize                  Literal
                isize                  Identifier
                        0b0101_0101    Literal                                                                                            */
	-1000isize as usize >> 3_usize,                                                                                                       /*
    -1000isize•as•usize•>>•3_usize    OperationExpression
    -1000isize•as•usize               ExpressionAsTypeCast
    -1000isize                        MinusExpression
     1000isize                        Literal
         isize                        Identifier
                           3_usize    Literal
                             usize    Identifier                                                                                          */
	2305843009213693827_usize,                                                                                                            /*
    2305843009213693827_usize    Literal
                        usize    Identifier                                                                                               */
	&42i32,                                                                                                                               /*
    &42i32    ReferenceExpression
     42i32    Literal
       i32    Identifier                                                                                                                  */
	a.0-1,                                                                                                                                /*
    a.0-1    OperationExpression
    a.0      MemberExpression
      0      Index
        1    Literal                                                                                                                      */
	a.0.1..2,                                                                                                                             /*
    a.0.1..2    RangeLiteral
    a.0.1       MemberExpression
    a.0         MemberExpression
      0         Index
        1       Index
           2    Literal                                                                                                                   */
	0.b0,                                                                                                                                 /*
    0.b0    MemberExpression
    0       Literal                                                                                                                       */
	'\u{10__FFFF}',                                                                                                                       /*
    '\u{10__FFFF}'    Literal                                                                                                             */
	"\u{10_F0FF__}foo\u{1_0_0_0__}",                                                                                                      /*
    "\u{10_F0FF__}foo\u{1_0_0_0__}"    Literal                                                                                            */
];                                                                                                                                        /*
];    </ExpressionStatement>
]     </ArrayLiteral>                                                                                                                     */

a!("\0=");                                                                                                                                /*
a!("\0=");    ExpressionStatement
a!("\0=")     MacroInvocation
   "\0="      Literal                                                                                                                     */

let _: f64 = 0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333;/*
let•_:•f64•=•0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333;    LetVariableDeclaration
    _                                                                                                                                                                                                                                                                                                                                                                                                 WildcardPattern
             0.3333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333     Literal*/
let _: f64 = 1234567890123456789012345678901234567890e-340;                                                                               /*
let•_:•f64•=•1234567890123456789012345678901234567890e-340;    LetVariableDeclaration
    _                                                          WildcardPattern
             1234567890123456789012345678901234567890e-340     Literal                                                                    */

fn main() {                                                                                                                               /*
fn•main()•{↲    <FunctionDeclaration>                                                                                                     */
    let s = "string                                                                                                                      "/*
    let•s•=•"string↲    <LetVariableDeclaration>
            "string↲    <Literal>                                                                                                        */"
literal";                                                                                                                                 /*
literal";    </LetVariableDeclaration>
literal"     </Literal>                                                                                                                   */

    let s = "literal with \                                                                                                              "/*
    let•s•=•"literal•with•\↲    <LetVariableDeclaration>
            "literal•with•\↲    <Literal>                                                                                                */"
             escaped newline";                                                                                                            /*
•••••••••••••escaped•newline";    </LetVariableDeclaration>
•••••••••••••escaped•newline"     </Literal>                                                                                              */

    let s = r"string                                                                                                                     "/*
    let•s•=•r"string↲    <LetVariableDeclaration>
            r"string↲    <Literal>                                                                                                      */r"
literal";                                                                                                                                 /*
literal";    </LetVariableDeclaration>
literal"     </Literal>                                                                                                                   */
    let s = br"byte string                                                                                                               "/*
    let•s•=•br"byte•string↲    <LetVariableDeclaration>
            br"byte•string↲    <Literal>                                                                                               */br"
literal";                                                                                                                                 /*
literal";    </LetVariableDeclaration>
literal"     </Literal>                                                                                                                   */
let text = "foo\r\nbar\n\nbaz\n";                                                                                                         /*
let•text•=•"foo\r\nbar\n\nbaz\n";    LetVariableDeclaration
           "foo\r\nbar\n\nbaz\n"     Literal                                                                                              */
let val = !((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//
                                                                                                                                          /*
let•val•=•!((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})//↲    <LetVariableDeclaration>
          !((|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];})       NotExpression
            (|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})=={&[..=..][..];}        ComparisonExpression
            (|(..):(_,_),__@_|__)((&*"\\",'🤔')/**/,{})                         CallExpression
             |(..):(_,_),__@_|__                                                ClosureFunctionExpression
              (..):(_,_)                                                        ClosureFunctionParameterDeclaration
              (..)                                                              TuplePattern
               ..                                                               RestPattern
                   (_,_)                                                        TypeTuple
                    _                                                           TypeInferred
                      _                                                         TypeInferred
                         __@_                                                   ClosureFunctionParameterDeclaration, PatternVariableDeclaration
                            _                                                   WildcardPattern
                                  (&*"\\",'🤔')                                 TupleLiteral
                                   &*"\\"                                       ReferenceExpression
                                    *"\\"                                       DereferenceExpression
                                     "\\"                                       Literal
                                          '🤔'                                  Literal
                                               /**/                             Comment
                                                    {}                          BlockExpression
                                                         {&[..=..][..];}        BlockExpression
                                                          &[..=..][..];         ExpressionStatement
                                                          &[..=..][..]          ReferenceExpression
                                                           [..=..][..]          MemberExpression
                                                           [..=..]              ArrayLiteral
                                                            ..=..               RangeLiteral
                                                               ..               RangeLiteral
                                                                   ..           RangeLiteral
                                                                         //     Comment                                                   */
    ;                                                                                                                                     /*
••••;    </LetVariableDeclaration>                                                                                                        */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */

(100).toString();                                                                                                                         /*
(100).toString();    ExpressionStatement
(100).toString()     CallExpression
 100                 Literal                                                                                                              */

0                                                                                                                                         /*
0    ExpressionStatement, Literal                                                                                                         */
1                                                                                                                                         /*
1    ExpressionStatement, Literal                                                                                                         */

0.1                                                                                                                                       /*
0.1    ExpressionStatement, Literal                                                                                                       */
1.1                                                                                                                                       /*
1.1↲    <ExpressionStatement>, <MemberExpression>
1.1     Literal                                                                                                                           */

.1                                                                                                                                        /*
.1    </ExpressionStatement>, </MemberExpression>
 1    Index                                                                                                                               */
1.                                                                                                                                        /*
1.    ExpressionStatement, Literal                                                                                                        */

0b1                                                                                                                                       /*
0b1    ExpressionStatement, Literal                                                                                                       */
0B1                                                                                                                                       /*
0B1    ExpressionStatement, Literal
 B1    Identifier                                                                                                                         */
0o1                                                                                                                                       /*
0o1    ExpressionStatement, Literal                                                                                                       */
0O1                                                                                                                                       /*
0O1    ExpressionStatement, Literal
 O1    Identifier                                                                                                                         */
0x1                                                                                                                                       /*
0x1    ExpressionStatement, Literal                                                                                                       */
0X1                                                                                                                                       /*
0X1    ExpressionStatement, Literal
 X1    Identifier                                                                                                                         */

0x123abcdef456ABCDEF                                                                                                                      /*
0x123abcdef456ABCDEF    ExpressionStatement, Literal                                                                                      */
0X123abcdef456ABCDEF                                                                                                                      /*
0X123abcdef456ABCDEF    ExpressionStatement, Literal
 X123abcdef456ABCDEF    Identifier                                                                                                        */
0xdeadbeef;                                                                                                                               /*
0xdeadbeef;    ExpressionStatement
0xdeadbeef     Literal                                                                                                                    */

0b111000                                                                                                                                  /*
0b111000    ExpressionStatement, Literal                                                                                                  */
0b000111                                                                                                                                  /*
0b000111    ExpressionStatement, Literal                                                                                                  */
0B111000                                                                                                                                  /*
0B111000    ExpressionStatement, Literal
 B111000    Identifier                                                                                                                    */
0B000111                                                                                                                                  /*
0B000111    ExpressionStatement, Literal
 B000111    Identifier                                                                                                                    */
0o111000                                                                                                                                  /*
0o111000    ExpressionStatement, Literal                                                                                                  */
0o000111                                                                                                                                  /*
0o000111    ExpressionStatement, Literal                                                                                                  */
0O111000                                                                                                                                  /*
0O111000    ExpressionStatement, Literal
 O111000    Identifier                                                                                                                    */
0O000111                                                                                                                                  /*
0O000111    ExpressionStatement, Literal
 O000111    Identifier                                                                                                                    */
0x111000                                                                                                                                  /*
0x111000    ExpressionStatement, Literal                                                                                                  */
0x000111                                                                                                                                  /*
0x000111    ExpressionStatement, Literal                                                                                                  */
0X111000                                                                                                                                  /*
0X111000    ExpressionStatement, Literal
 X111000    Identifier                                                                                                                    */
0X000111                                                                                                                                  /*
0X000111    ExpressionStatement, Literal
 X000111    Identifier                                                                                                                    */

1e1                                                                                                                                       /*
1e1    ExpressionStatement, Literal                                                                                                       */
1e+1                                                                                                                                      /*
1e+1    ExpressionStatement, Literal                                                                                                      */
1e-1                                                                                                                                      /*
1e-1    ExpressionStatement, Literal                                                                                                      */
1.e1                                                                                                                                      /*
1.e1↲    <ExpressionStatement>, <MemberExpression>
1.e1     MemberExpression
1        Literal                                                                                                                          */
.1e1                                                                                                                                      /*
.1      </ExpressionStatement>, </MemberExpression>
 1      Index
  e1    ExpressionStatement                                                                                                               */
1.1e1                                                                                                                                     /*
1.1e1    ExpressionStatement, Literal                                                                                                     */
1.1e0010                                                                                                                                  /*
1.1e0010↲    <ExpressionStatement>, <MemberExpression>
1.1e0010     Literal                                                                                                                      */
.1e+0010                                                                                                                                  /*
.1           </ExpressionStatement>, </MemberExpression>
 1           Index
  e+0010↲    <ExpressionStatement>, <OperationExpression>
    0010↲    <MemberExpression>
    0010     Literal                                                                                                                      */
.1e-0010                                                                                                                                  /*
.1          </ExpressionStatement>, </OperationExpression>
 1          Index
  e-0010    ExpressionStatement, OperationExpression
    0010    Literal                                                                                                                       */

1E1                                                                                                                                       /*
1E1    ExpressionStatement, Literal                                                                                                       */
1E+1                                                                                                                                      /*
1E+1    ExpressionStatement, Literal                                                                                                      */
1E-1                                                                                                                                      /*
1E-1    ExpressionStatement, Literal                                                                                                      */
1.E1                                                                                                                                      /*
1.E1↲    <ExpressionStatement>, <MemberExpression>
1.E1     MemberExpression
1        Literal                                                                                                                          */
.1E1                                                                                                                                      /*
.1      </ExpressionStatement>, </MemberExpression>
 1      Index
  E1    ExpressionStatement                                                                                                               */
1.1E1                                                                                                                                     /*
1.1E1    ExpressionStatement, Literal                                                                                                     */
1.1E0010                                                                                                                                  /*
1.1E0010↲    <ExpressionStatement>, <MemberExpression>
1.1E0010     Literal                                                                                                                      */
.1E+0010                                                                                                                                  /*
.1           </ExpressionStatement>, </MemberExpression>
 1           Index
  E+0010↲    <ExpressionStatement>, <OperationExpression>
    0010↲    <MemberExpression>
    0010     Literal                                                                                                                      */
.1E-0010                                                                                                                                  /*
.1          </ExpressionStatement>, </OperationExpression>
 1          Index
  E-0010    ExpressionStatement, OperationExpression
    0010    Literal                                                                                                                       */

0.5e0                                                                                                                                     /*
0.5e0    ExpressionStatement, Literal                                                                                                     */
0.5e00                                                                                                                                    /*
0.5e00    ExpressionStatement, Literal                                                                                                    */
0.5e+0                                                                                                                                    /*
0.5e+0    ExpressionStatement, Literal                                                                                                    */
0.5e+00                                                                                                                                   /*
0.5e+00    ExpressionStatement, Literal                                                                                                   */
0.5e-0                                                                                                                                    /*
0.5e-0    ExpressionStatement, Literal                                                                                                    */
0.5e-00                                                                                                                                   /*
0.5e-00    ExpressionStatement, Literal                                                                                                   */

1                                                                                                                                         /*
1    ExpressionStatement, Literal                                                                                                         */
1.00500                                                                                                                                   /*
1.00500    ExpressionStatement, Literal                                                                                                   */
1.0                                                                                                                                       /*
1.0    ExpressionStatement, Literal                                                                                                       */
1.5                                                                                                                                       /*
1.5    ExpressionStatement, Literal                                                                                                       */
1.50                                                                                                                                      /*
1.50    ExpressionStatement, Literal                                                                                                      */
0                                                                                                                                         /*
0    ExpressionStatement, Literal                                                                                                         */
0.00500                                                                                                                                   /*
0.00500    ExpressionStatement, Literal                                                                                                   */
0.0                                                                                                                                       /*
0.0    ExpressionStatement, Literal                                                                                                       */
0.0000                                                                                                                                    /*
0.0000↲    <ExpressionStatement>, <MemberExpression>
0.0000     Literal                                                                                                                        */
.0                                                                                                                                        /*
.0    </ExpressionStatement>, </MemberExpression>
 0    Index                                                                                                                               */
500600.001230045000                                                                                                                       /*
500600.001230045000    ExpressionStatement, Literal                                                                                       */
1.00500e60                                                                                                                                /*
1.00500e60    ExpressionStatement, Literal                                                                                                */
1.0e60                                                                                                                                    /*
1.0e60    ExpressionStatement, Literal                                                                                                    */
0.00500e60                                                                                                                                /*
0.00500e60    ExpressionStatement, Literal                                                                                                */
0.0e60                                                                                                                                    /*
0.0e60    ExpressionStatement, Literal                                                                                                    */
0.0000e60                                                                                                                                 /*
0.0000e60↲    <ExpressionStatement>, <MemberExpression>
0.0000e60     Literal                                                                                                                     */
.0e60                                                                                                                                     /*
.0       </ExpressionStatement>, </MemberExpression>
 0       Index
  e60    ExpressionStatement                                                                                                              */
0.e60                                                                                                                                     /*
0.e60    ExpressionStatement, MemberExpression
0        Literal                                                                                                                          */
0e60                                                                                                                                      /*
0e60    ExpressionStatement, Literal                                                                                                      */
500600.001230045000e60                                                                                                                    /*
500600.001230045000e60    ExpressionStatement, Literal                                                                                    */
10                                                                                                                                        /*
10    ExpressionStatement, Literal                                                                                                        */
9700                                                                                                                                      /*
9700    ExpressionStatement, Literal                                                                                                      */
10e100                                                                                                                                    /*
10e100    ExpressionStatement, Literal                                                                                                    */

1_1                                                                                                                                       /*
1_1    ExpressionStatement, Literal                                                                                                       */
1_1.1_1                                                                                                                                   /*
1_1.1_1    ExpressionStatement, Literal                                                                                                   */
0o1_1                                                                                                                                     /*
0o1_1    ExpressionStatement, Literal                                                                                                     */
0o0_11                                                                                                                                    /*
0o0_11    ExpressionStatement, Literal                                                                                                    */
1.1_0_1e1                                                                                                                                 /*
1.1_0_1e1    ExpressionStatement, Literal                                                                                                 */
1.1_0_1E1                                                                                                                                 /*
1.1_0_1E1↲    <ExpressionStatement>, <MemberExpression>
1.1_0_1E1     Literal                                                                                                                     */
.1_1                                                                                                                                      /*
.1      </ExpressionStatement>, </MemberExpression>
 1      Index
  _1    ExpressionStatement                                                                                                               */
0x1_1                                                                                                                                     /*
0x1_1    ExpressionStatement, Literal                                                                                                     */
0xa_1                                                                                                                                     /*
0xa_1    ExpressionStatement, Literal                                                                                                     */
0xA_1                                                                                                                                     /*
0xA_1    ExpressionStatement, Literal                                                                                                     */
0b01_1                                                                                                                                    /*
0b01_1    ExpressionStatement, Literal                                                                                                    */
0b0_1_1                                                                                                                                   /*
0b0_1_1    ExpressionStatement, Literal                                                                                                   */

// Discarded Nodes: 8
// Parsed Nodes: 565
// state_rollbacks: 0
// Total '.charCodeAt()' calls: 4054 (17% re-reads)
// Unnecessary 'skip_whitespace()' calls: 170
// source: "../../samples/expressions/literal.rs"