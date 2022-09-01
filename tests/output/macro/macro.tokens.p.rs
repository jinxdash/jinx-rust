macro_rules! a {                                                                                                                          /*
macro_rules!•a•{↲    <Program>
macro_rules!•a•{↲    <Program.ast{dk: "None"}>
macro_rules!•a•{↲    <MacroRulesDeclaration>
               {↲    <MacroRulesDeclaration.rules{dk: "{}"}>                                                                              */
	( + ) => { + }; ( += ) => { += }; ( & ) => { & }; ( && ) => { && };                                                                   /*
	(•+•)•=>•{•+•}                                                        MacroRuleDeclaration
	(•+•)                                                                 MacroRuleDeclaration.match{dk: "()"}
	  +                                                                   PunctuationToken{tk: "+"}
	         {•+•}                                                        MacroRuleDeclaration.transform{dk: "{}"}
	           +                                                          PunctuationToken{tk: "+"}
	                (•+=•)•=>•{•+=•}                                      MacroRuleDeclaration
	                (•+=•)                                                MacroRuleDeclaration.match{dk: "()"}
	                  +=                                                  PunctuationToken{tk: "+="}
	                          {•+=•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                            +=                                        PunctuationToken{tk: "+="}
	                                  (•&•)•=>•{•&•}                      MacroRuleDeclaration
	                                  (•&•)                               MacroRuleDeclaration.match{dk: "()"}
	                                    &                                 PunctuationToken{tk: "&"}
	                                           {•&•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                             &                        PunctuationToken{tk: "&"}
	                                                  (•&&•)•=>•{•&&•}    MacroRuleDeclaration
	                                                  (•&&•)              MacroRuleDeclaration.match{dk: "()"}
	                                                    &&                PunctuationToken{tk: "&&"}
	                                                            {•&&•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                              &&      PunctuationToken{tk: "&&"}                                      */
	( &= ) => { &= }; ( @ ) => { @ }; ( ! ) => { ! }; ( ^ ) => { ^ };                                                                     /*
	(•&=•)•=>•{•&=•}                                                    MacroRuleDeclaration
	(•&=•)                                                              MacroRuleDeclaration.match{dk: "()"}
	  &=                                                                PunctuationToken{tk: "&="}
	          {•&=•}                                                    MacroRuleDeclaration.transform{dk: "{}"}
	            &=                                                      PunctuationToken{tk: "&="}
	                  (•@•)•=>•{•@•}                                    MacroRuleDeclaration
	                  (•@•)                                             MacroRuleDeclaration.match{dk: "()"}
	                    @                                               PunctuationToken{tk: "@"}
	                           {•@•}                                    MacroRuleDeclaration.transform{dk: "{}"}
	                             @                                      PunctuationToken{tk: "@"}
	                                  (•!•)•=>•{•!•}                    MacroRuleDeclaration
	                                  (•!•)                             MacroRuleDeclaration.match{dk: "()"}
	                                    !                               PunctuationToken{tk: "!"}
	                                           {•!•}                    MacroRuleDeclaration.transform{dk: "{}"}
	                                             !                      PunctuationToken{tk: "!"}
	                                                  (•^•)•=>•{•^•}    MacroRuleDeclaration
	                                                  (•^•)             MacroRuleDeclaration.match{dk: "()"}
	                                                    ^               PunctuationToken{tk: "^"}
	                                                           {•^•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                             ^      PunctuationToken{tk: "^"}                                         */
	( ^= ) => { ^= }; ( : ) => { : }; ( :: ) => { :: }; ( , ) => { , };                                                                   /*
	(•^=•)•=>•{•^=•}                                                      MacroRuleDeclaration
	(•^=•)                                                                MacroRuleDeclaration.match{dk: "()"}
	  ^=                                                                  PunctuationToken{tk: "^="}
	          {•^=•}                                                      MacroRuleDeclaration.transform{dk: "{}"}
	            ^=                                                        PunctuationToken{tk: "^="}
	                  (•:•)•=>•{•:•}                                      MacroRuleDeclaration
	                  (•:•)                                               MacroRuleDeclaration.match{dk: "()"}
	                    :                                                 PunctuationToken{tk: ":"}
	                           {•:•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                             :                                        PunctuationToken{tk: ":"}
	                                  (•::•)•=>•{•::•}                    MacroRuleDeclaration
	                                  (•::•)                              MacroRuleDeclaration.match{dk: "()"}
	                                    ::                                PunctuationToken{tk: "::"}
	                                            {•::•}                    MacroRuleDeclaration.transform{dk: "{}"}
	                                              ::                      PunctuationToken{tk: "::"}
	                                                    (•,•)•=>•{•,•}    MacroRuleDeclaration
	                                                    (•,•)             MacroRuleDeclaration.match{dk: "()"}
	                                                      ,               PunctuationToken{tk: ","}
	                                                             {•,•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                               ,      PunctuationToken{tk: ","}                                       */
	( / ) => { / }; ( /= ) => { /= }; ( . ) => { . }; ( .. ) => { .. };                                                                   /*
	(•/•)•=>•{•/•}                                                        MacroRuleDeclaration
	(•/•)                                                                 MacroRuleDeclaration.match{dk: "()"}
	  /                                                                   PunctuationToken{tk: "/"}
	         {•/•}                                                        MacroRuleDeclaration.transform{dk: "{}"}
	           /                                                          PunctuationToken{tk: "/"}
	                (•/=•)•=>•{•/=•}                                      MacroRuleDeclaration
	                (•/=•)                                                MacroRuleDeclaration.match{dk: "()"}
	                  /=                                                  PunctuationToken{tk: "/="}
	                          {•/=•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                            /=                                        PunctuationToken{tk: "/="}
	                                  (•.•)•=>•{•.•}                      MacroRuleDeclaration
	                                  (•.•)                               MacroRuleDeclaration.match{dk: "()"}
	                                    .                                 PunctuationToken{tk: "."}
	                                           {•.•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                             .                        PunctuationToken{tk: "."}
	                                                  (•..•)•=>•{•..•}    MacroRuleDeclaration
	                                                  (•..•)              MacroRuleDeclaration.match{dk: "()"}
	                                                    ..                PunctuationToken{tk: ".."}
	                                                            {•..•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                              ..      PunctuationToken{tk: ".."}                                      */
	( ... ) => { ... }; ( ..= ) => { ..= }; ( = ) => { = }; ( == ) => { == };                                                             /*
	(•...•)•=>•{•...•}                                                          MacroRuleDeclaration
	(•...•)                                                                     MacroRuleDeclaration.match{dk: "()"}
	  ...                                                                       PunctuationToken{tk: "..."}
	           {•...•}                                                          MacroRuleDeclaration.transform{dk: "{}"}
	             ...                                                            PunctuationToken{tk: "..."}
	                    (•..=•)•=>•{•..=•}                                      MacroRuleDeclaration
	                    (•..=•)                                                 MacroRuleDeclaration.match{dk: "()"}
	                      ..=                                                   PunctuationToken{tk: "..="}
	                               {•..=•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                                 ..=                                        PunctuationToken{tk: "..="}
	                                        (•=•)•=>•{•=•}                      MacroRuleDeclaration
	                                        (•=•)                               MacroRuleDeclaration.match{dk: "()"}
	                                          =                                 PunctuationToken{tk: "="}
	                                                 {•=•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                                   =                        PunctuationToken{tk: "="}
	                                                        (•==•)•=>•{•==•}    MacroRuleDeclaration
	                                                        (•==•)              MacroRuleDeclaration.match{dk: "()"}
	                                                          ==                PunctuationToken{tk: "=="}
	                                                                  {•==•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                    ==      PunctuationToken{tk: "=="}                                */
	( >= ) => { >= }; ( > ) => { > }; ( <= ) => { <= }; ( < ) => { < };                                                                   /*
	(•>=•)•=>•{•>=•}                                                      MacroRuleDeclaration
	(•>=•)                                                                MacroRuleDeclaration.match{dk: "()"}
	  >=                                                                  PunctuationToken{tk: ">="}
	          {•>=•}                                                      MacroRuleDeclaration.transform{dk: "{}"}
	            >=                                                        PunctuationToken{tk: ">="}
	                  (•>•)•=>•{•>•}                                      MacroRuleDeclaration
	                  (•>•)                                               MacroRuleDeclaration.match{dk: "()"}
	                    >                                                 PunctuationToken{tk: ">"}
	                           {•>•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                             >                                        PunctuationToken{tk: ">"}
	                                  (•<=•)•=>•{•<=•}                    MacroRuleDeclaration
	                                  (•<=•)                              MacroRuleDeclaration.match{dk: "()"}
	                                    <=                                PunctuationToken{tk: "<="}
	                                            {•<=•}                    MacroRuleDeclaration.transform{dk: "{}"}
	                                              <=                      PunctuationToken{tk: "<="}
	                                                    (•<•)•=>•{•<•}    MacroRuleDeclaration
	                                                    (•<•)             MacroRuleDeclaration.match{dk: "()"}
	                                                      <               PunctuationToken{tk: "<"}
	                                                             {•<•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                               <      PunctuationToken{tk: "<"}                                       */
	( *= ) => { *= }; ( != ) => { != }; ( | ) => { | }; ( |= ) => { |= };                                                                 /*
	(•*=•)•=>•{•*=•}                                                        MacroRuleDeclaration
	(•*=•)                                                                  MacroRuleDeclaration.match{dk: "()"}
	  *=                                                                    PunctuationToken{tk: "*="}
	          {•*=•}                                                        MacroRuleDeclaration.transform{dk: "{}"}
	            *=                                                          PunctuationToken{tk: "*="}
	                  (•!=•)•=>•{•!=•}                                      MacroRuleDeclaration
	                  (•!=•)                                                MacroRuleDeclaration.match{dk: "()"}
	                    !=                                                  PunctuationToken{tk: "!="}
	                            {•!=•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                              !=                                        PunctuationToken{tk: "!="}
	                                    (•|•)•=>•{•|•}                      MacroRuleDeclaration
	                                    (•|•)                               MacroRuleDeclaration.match{dk: "()"}
	                                      |                                 PunctuationToken{tk: "|"}
	                                             {•|•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                               |                        PunctuationToken{tk: "|"}
	                                                    (•|=•)•=>•{•|=•}    MacroRuleDeclaration
	                                                    (•|=•)              MacroRuleDeclaration.match{dk: "()"}
	                                                      |=                PunctuationToken{tk: "|="}
	                                                              {•|=•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                |=      PunctuationToken{tk: "|="}                                    */
	( || ) => { || }; ( # ) => { # }; ( ? ) => { ? }; ( -> ) => { -> };                                                                   /*
	(•||•)•=>•{•||•}                                                      MacroRuleDeclaration
	(•||•)                                                                MacroRuleDeclaration.match{dk: "()"}
	  ||                                                                  PunctuationToken{tk: "||"}
	          {•||•}                                                      MacroRuleDeclaration.transform{dk: "{}"}
	            ||                                                        PunctuationToken{tk: "||"}
	                  (•#•)•=>•{•#•}                                      MacroRuleDeclaration
	                  (•#•)                                               MacroRuleDeclaration.match{dk: "()"}
	                    #                                                 PunctuationToken{tk: "#"}
	                           {•#•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                             #                                        PunctuationToken{tk: "#"}
	                                  (•?•)•=>•{•?•}                      MacroRuleDeclaration
	                                  (•?•)                               MacroRuleDeclaration.match{dk: "()"}
	                                    ?                                 PunctuationToken{tk: "?"}
	                                           {•?•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                             ?                        PunctuationToken{tk: "?"}
	                                                  (•->•)•=>•{•->•}    MacroRuleDeclaration
	                                                  (•->•)              MacroRuleDeclaration.match{dk: "()"}
	                                                    ->                PunctuationToken{tk: "->"}
	                                                            {•->•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                              ->      PunctuationToken{tk: "->"}                                      */
	( <- ) => { <- }; ( % ) => { % }; ( %= ) => { %= }; ( => ) => { => };                                                                 /*
	(•<-•)•=>•{•<-•}                                                        MacroRuleDeclaration
	(•<-•)                                                                  MacroRuleDeclaration.match{dk: "()"}
	  <                                                                     PunctuationToken{tk: "<"}
	   -                                                                    PunctuationToken{tk: "-"}
	          {•<-•}                                                        MacroRuleDeclaration.transform{dk: "{}"}
	            <                                                           PunctuationToken{tk: "<"}
	             -                                                          PunctuationToken{tk: "-"}
	                  (•%•)•=>•{•%•}                                        MacroRuleDeclaration
	                  (•%•)                                                 MacroRuleDeclaration.match{dk: "()"}
	                    %                                                   PunctuationToken{tk: "%"}
	                           {•%•}                                        MacroRuleDeclaration.transform{dk: "{}"}
	                             %                                          PunctuationToken{tk: "%"}
	                                  (•%=•)•=>•{•%=•}                      MacroRuleDeclaration
	                                  (•%=•)                                MacroRuleDeclaration.match{dk: "()"}
	                                    %=                                  PunctuationToken{tk: "%="}
	                                            {•%=•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                              %=                        PunctuationToken{tk: "%="}
	                                                    (•=>•)•=>•{•=>•}    MacroRuleDeclaration
	                                                    (•=>•)              MacroRuleDeclaration.match{dk: "()"}
	                                                      =>                PunctuationToken{tk: "=>"}
	                                                              {•=>•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                =>      PunctuationToken{tk: "=>"}                                    */
	( ; ) => { ; }; ( << ) => { << }; ( <<= ) => { <<= }; ( >> ) => { >> };                                                               /*
	(•;•)•=>•{•;•}                                                            MacroRuleDeclaration
	(•;•)                                                                     MacroRuleDeclaration.match{dk: "()"}
	  ;                                                                       PunctuationToken{tk: ";"}
	         {•;•}                                                            MacroRuleDeclaration.transform{dk: "{}"}
	           ;                                                              PunctuationToken{tk: ";"}
	                (•<<•)•=>•{•<<•}                                          MacroRuleDeclaration
	                (•<<•)                                                    MacroRuleDeclaration.match{dk: "()"}
	                  <<                                                      PunctuationToken{tk: "<<"}
	                          {•<<•}                                          MacroRuleDeclaration.transform{dk: "{}"}
	                            <<                                            PunctuationToken{tk: "<<"}
	                                  (•<<=•)•=>•{•<<=•}                      MacroRuleDeclaration
	                                  (•<<=•)                                 MacroRuleDeclaration.match{dk: "()"}
	                                    <<=                                   PunctuationToken{tk: "<<="}
	                                             {•<<=•}                      MacroRuleDeclaration.transform{dk: "{}"}
	                                               <<=                        PunctuationToken{tk: "<<="}
	                                                      (•>>•)•=>•{•>>•}    MacroRuleDeclaration
	                                                      (•>>•)              MacroRuleDeclaration.match{dk: "()"}
	                                                        >>                PunctuationToken{tk: ">>"}
	                                                                {•>>•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                  >>      PunctuationToken{tk: ">>"}                                  */
	( >>= ) => { >>= }; ( * ) => { * }; ( - ) => { - }; ( -= ) => { -= }; ( ~ ) => { ~ };                                                 /*
	(•>>=•)•=>•{•>>=•}                                                                      MacroRuleDeclaration
	(•>>=•)                                                                                 MacroRuleDeclaration.match{dk: "()"}
	  >>=                                                                                   PunctuationToken{tk: ">>="}
	           {•>>=•}                                                                      MacroRuleDeclaration.transform{dk: "{}"}
	             >>=                                                                        PunctuationToken{tk: ">>="}
	                    (•*•)•=>•{•*•}                                                      MacroRuleDeclaration
	                    (•*•)                                                               MacroRuleDeclaration.match{dk: "()"}
	                      *                                                                 PunctuationToken{tk: "*"}
	                             {•*•}                                                      MacroRuleDeclaration.transform{dk: "{}"}
	                               *                                                        PunctuationToken{tk: "*"}
	                                    (•-•)•=>•{•-•}                                      MacroRuleDeclaration
	                                    (•-•)                                               MacroRuleDeclaration.match{dk: "()"}
	                                      -                                                 PunctuationToken{tk: "-"}
	                                             {•-•}                                      MacroRuleDeclaration.transform{dk: "{}"}
	                                               -                                        PunctuationToken{tk: "-"}
	                                                    (•-=•)•=>•{•-=•}                    MacroRuleDeclaration
	                                                    (•-=•)                              MacroRuleDeclaration.match{dk: "()"}
	                                                      -=                                PunctuationToken{tk: "-="}
	                                                              {•-=•}                    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                -=                      PunctuationToken{tk: "-="}
	                                                                      (•~•)•=>•{•~•}    MacroRuleDeclaration
	                                                                      (•~•)             MacroRuleDeclaration.match{dk: "()"}
	                                                                        ~               PunctuationToken{tk: "~"}
	                                                                               {•~•}    MacroRuleDeclaration.transform{dk: "{}"}
	                                                                                 ~      PunctuationToken{tk: "~"}                     */
}                                                                                                                                         /*
}    </MacroRulesDeclaration.rules>
}    </MacroRulesDeclaration>                                                                                                             */


a!{ + } a!{ += } a!{ & } a!{ && } a!{ &= } a!{ @ } a!{ ! } a!{ ^ } a!{ ^= }                                                               /*
a!{•+•}                                                                        ExpressionStatement{!semi}, MacroInvocation
  {•+•}                                                                        MacroInvocation.segments{dk: "{}"}
    +                                                                          PunctuationToken{tk: "+"}
        a!{•+=•}                                                               ExpressionStatement{!semi}, MacroInvocation
          {•+=•}                                                               MacroInvocation.segments{dk: "{}"}
            +=                                                                 PunctuationToken{tk: "+="}
                 a!{•&•}                                                       ExpressionStatement{!semi}, MacroInvocation
                   {•&•}                                                       MacroInvocation.segments{dk: "{}"}
                     &                                                         PunctuationToken{tk: "&"}
                         a!{•&&•}                                              ExpressionStatement{!semi}, MacroInvocation
                           {•&&•}                                              MacroInvocation.segments{dk: "{}"}
                             &&                                                PunctuationToken{tk: "&&"}
                                  a!{•&=•}                                     ExpressionStatement{!semi}, MacroInvocation
                                    {•&=•}                                     MacroInvocation.segments{dk: "{}"}
                                      &=                                       PunctuationToken{tk: "&="}
                                           a!{•@•}                             ExpressionStatement{!semi}, MacroInvocation
                                             {•@•}                             MacroInvocation.segments{dk: "{}"}
                                               @                               PunctuationToken{tk: "@"}
                                                   a!{•!•}                     ExpressionStatement{!semi}, MacroInvocation
                                                     {•!•}                     MacroInvocation.segments{dk: "{}"}
                                                       !                       PunctuationToken{tk: "!"}
                                                           a!{•^•}             ExpressionStatement{!semi}, MacroInvocation
                                                             {•^•}             MacroInvocation.segments{dk: "{}"}
                                                               ^               PunctuationToken{tk: "^"}
                                                                   a!{•^=•}    ExpressionStatement{!semi}, MacroInvocation
                                                                     {•^=•}    MacroInvocation.segments{dk: "{}"}
                                                                       ^=      PunctuationToken{tk: "^="}                                 */
a!{ : } a!{ :: } a!{ , } a!{ / } a!{ /= } a!{ . } a!{ .. } a!{ ... } a!{ ..= }                                                            /*
a!{•:•}                                                                           ExpressionStatement{!semi}, MacroInvocation
  {•:•}                                                                           MacroInvocation.segments{dk: "{}"}
    :                                                                             PunctuationToken{tk: ":"}
        a!{•::•}                                                                  ExpressionStatement{!semi}, MacroInvocation
          {•::•}                                                                  MacroInvocation.segments{dk: "{}"}
            ::                                                                    PunctuationToken{tk: "::"}
                 a!{•,•}                                                          ExpressionStatement{!semi}, MacroInvocation
                   {•,•}                                                          MacroInvocation.segments{dk: "{}"}
                     ,                                                            PunctuationToken{tk: ","}
                         a!{•/•}                                                  ExpressionStatement{!semi}, MacroInvocation
                           {•/•}                                                  MacroInvocation.segments{dk: "{}"}
                             /                                                    PunctuationToken{tk: "/"}
                                 a!{•/=•}                                         ExpressionStatement{!semi}, MacroInvocation
                                   {•/=•}                                         MacroInvocation.segments{dk: "{}"}
                                     /=                                           PunctuationToken{tk: "/="}
                                          a!{•.•}                                 ExpressionStatement{!semi}, MacroInvocation
                                            {•.•}                                 MacroInvocation.segments{dk: "{}"}
                                              .                                   PunctuationToken{tk: "."}
                                                  a!{•..•}                        ExpressionStatement{!semi}, MacroInvocation
                                                    {•..•}                        MacroInvocation.segments{dk: "{}"}
                                                      ..                          PunctuationToken{tk: ".."}
                                                           a!{•...•}              ExpressionStatement{!semi}, MacroInvocation
                                                             {•...•}              MacroInvocation.segments{dk: "{}"}
                                                               ...                PunctuationToken{tk: "..."}
                                                                     a!{•..=•}    ExpressionStatement{!semi}, MacroInvocation
                                                                       {•..=•}    MacroInvocation.segments{dk: "{}"}
                                                                         ..=      PunctuationToken{tk: "..="}                             */
a!{ = } a!{ == } a!{ >= } a!{ > } a!{ <= } a!{ < } a!{ *= } a!{ != } a!{ | }                                                              /*
a!{•=•}                                                                         ExpressionStatement{!semi}, MacroInvocation
  {•=•}                                                                         MacroInvocation.segments{dk: "{}"}
    =                                                                           PunctuationToken{tk: "="}
        a!{•==•}                                                                ExpressionStatement{!semi}, MacroInvocation
          {•==•}                                                                MacroInvocation.segments{dk: "{}"}
            ==                                                                  PunctuationToken{tk: "=="}
                 a!{•>=•}                                                       ExpressionStatement{!semi}, MacroInvocation
                   {•>=•}                                                       MacroInvocation.segments{dk: "{}"}
                     >=                                                         PunctuationToken{tk: ">="}
                          a!{•>•}                                               ExpressionStatement{!semi}, MacroInvocation
                            {•>•}                                               MacroInvocation.segments{dk: "{}"}
                              >                                                 PunctuationToken{tk: ">"}
                                  a!{•<=•}                                      ExpressionStatement{!semi}, MacroInvocation
                                    {•<=•}                                      MacroInvocation.segments{dk: "{}"}
                                      <=                                        PunctuationToken{tk: "<="}
                                           a!{•<•}                              ExpressionStatement{!semi}, MacroInvocation
                                             {•<•}                              MacroInvocation.segments{dk: "{}"}
                                               <                                PunctuationToken{tk: "<"}
                                                   a!{•*=•}                     ExpressionStatement{!semi}, MacroInvocation
                                                     {•*=•}                     MacroInvocation.segments{dk: "{}"}
                                                       *=                       PunctuationToken{tk: "*="}
                                                            a!{•!=•}            ExpressionStatement{!semi}, MacroInvocation
                                                              {•!=•}            MacroInvocation.segments{dk: "{}"}
                                                                !=              PunctuationToken{tk: "!="}
                                                                     a!{•|•}    ExpressionStatement{!semi}, MacroInvocation
                                                                       {•|•}    MacroInvocation.segments{dk: "{}"}
                                                                         |      PunctuationToken{tk: "|"}                                 */
a!{ |= } a!{ || } a!{ # } a!{ ? } a!{ -> } a!{ <- } a!{ % } a!{ %= } a!{ => }                                                             /*
a!{•|=•}                                                                         ExpressionStatement{!semi}, MacroInvocation
  {•|=•}                                                                         MacroInvocation.segments{dk: "{}"}
    |=                                                                           PunctuationToken{tk: "|="}
         a!{•||•}                                                                ExpressionStatement{!semi}, MacroInvocation
           {•||•}                                                                MacroInvocation.segments{dk: "{}"}
             ||                                                                  PunctuationToken{tk: "||"}
                  a!{•#•}                                                        ExpressionStatement{!semi}, MacroInvocation
                    {•#•}                                                        MacroInvocation.segments{dk: "{}"}
                      #                                                          PunctuationToken{tk: "#"}
                          a!{•?•}                                                ExpressionStatement{!semi}, MacroInvocation
                            {•?•}                                                MacroInvocation.segments{dk: "{}"}
                              ?                                                  PunctuationToken{tk: "?"}
                                  a!{•->•}                                       ExpressionStatement{!semi}, MacroInvocation
                                    {•->•}                                       MacroInvocation.segments{dk: "{}"}
                                      ->                                         PunctuationToken{tk: "->"}
                                           a!{•<-•}                              ExpressionStatement{!semi}, MacroInvocation
                                             {•<-•}                              MacroInvocation.segments{dk: "{}"}
                                               <                                 PunctuationToken{tk: "<"}
                                                -                                PunctuationToken{tk: "-"}
                                                    a!{•%•}                      ExpressionStatement{!semi}, MacroInvocation
                                                      {•%•}                      MacroInvocation.segments{dk: "{}"}
                                                        %                        PunctuationToken{tk: "%"}
                                                            a!{•%=•}             ExpressionStatement{!semi}, MacroInvocation
                                                              {•%=•}             MacroInvocation.segments{dk: "{}"}
                                                                %=               PunctuationToken{tk: "%="}
                                                                     a!{•=>•}    ExpressionStatement{!semi}, MacroInvocation
                                                                       {•=>•}    MacroInvocation.segments{dk: "{}"}
                                                                         =>      PunctuationToken{tk: "=>"}                               */
a!{ ; } a!{ << } a!{ <<= } a!{ >> } a!{ >>= } a!{ * } a!{ - } a!{ -= } a!{ ~ }                                                            /*
a!{•;•}                                                                           ExpressionStatement{!semi}, MacroInvocation
  {•;•}                                                                           MacroInvocation.segments{dk: "{}"}
    ;                                                                             PunctuationToken{tk: ";"}
        a!{•<<•}                                                                  ExpressionStatement{!semi}, MacroInvocation
          {•<<•}                                                                  MacroInvocation.segments{dk: "{}"}
            <<                                                                    PunctuationToken{tk: "<<"}
                 a!{•<<=•}                                                        ExpressionStatement{!semi}, MacroInvocation
                   {•<<=•}                                                        MacroInvocation.segments{dk: "{}"}
                     <<=                                                          PunctuationToken{tk: "<<="}
                           a!{•>>•}                                               ExpressionStatement{!semi}, MacroInvocation
                             {•>>•}                                               MacroInvocation.segments{dk: "{}"}
                               >>                                                 PunctuationToken{tk: ">>"}
                                    a!{•>>=•}                                     ExpressionStatement{!semi}, MacroInvocation
                                      {•>>=•}                                     MacroInvocation.segments{dk: "{}"}
                                        >>=                                       PunctuationToken{tk: ">>="}
                                              a!{•*•}                             ExpressionStatement{!semi}, MacroInvocation
                                                {•*•}                             MacroInvocation.segments{dk: "{}"}
                                                  *                               PunctuationToken{tk: "*"}
                                                      a!{•-•}                     ExpressionStatement{!semi}, MacroInvocation
                                                        {•-•}                     MacroInvocation.segments{dk: "{}"}
                                                          -                       PunctuationToken{tk: "-"}
                                                              a!{•-=•}            ExpressionStatement{!semi}, MacroInvocation
                                                                {•-=•}            MacroInvocation.segments{dk: "{}"}
                                                                  -=              PunctuationToken{tk: "-="}
                                                                       a!{•~•}    ExpressionStatement{!semi}, MacroInvocation
                                                                         {•~•}    MacroInvocation.segments{dk: "{}"}
                                                                           ~      PunctuationToken{tk: "~"}                               */


b![ + ] b![ += ] b![ & ] b![ && ] b![ &= ] b![ @ ] b![ ! ] b![ ^ ] b![ ^= ]                                                               /*
b![•+•]                                                                        ExpressionStatement{!semi}, MacroInvocation
  [•+•]                                                                        MacroInvocation.segments{dk: "[]"}
    +                                                                          PunctuationToken{tk: "+"}
        b![•+=•]                                                               ExpressionStatement{!semi}, MacroInvocation
          [•+=•]                                                               MacroInvocation.segments{dk: "[]"}
            +=                                                                 PunctuationToken{tk: "+="}
                 b![•&•]                                                       ExpressionStatement{!semi}, MacroInvocation
                   [•&•]                                                       MacroInvocation.segments{dk: "[]"}
                     &                                                         PunctuationToken{tk: "&"}
                         b![•&&•]                                              ExpressionStatement{!semi}, MacroInvocation
                           [•&&•]                                              MacroInvocation.segments{dk: "[]"}
                             &&                                                PunctuationToken{tk: "&&"}
                                  b![•&=•]                                     ExpressionStatement{!semi}, MacroInvocation
                                    [•&=•]                                     MacroInvocation.segments{dk: "[]"}
                                      &=                                       PunctuationToken{tk: "&="}
                                           b![•@•]                             ExpressionStatement{!semi}, MacroInvocation
                                             [•@•]                             MacroInvocation.segments{dk: "[]"}
                                               @                               PunctuationToken{tk: "@"}
                                                   b![•!•]                     ExpressionStatement{!semi}, MacroInvocation
                                                     [•!•]                     MacroInvocation.segments{dk: "[]"}
                                                       !                       PunctuationToken{tk: "!"}
                                                           b![•^•]             ExpressionStatement{!semi}, MacroInvocation
                                                             [•^•]             MacroInvocation.segments{dk: "[]"}
                                                               ^               PunctuationToken{tk: "^"}
                                                                   b![•^=•]    ExpressionStatement{!semi}, MacroInvocation
                                                                     [•^=•]    MacroInvocation.segments{dk: "[]"}
                                                                       ^=      PunctuationToken{tk: "^="}                                 */
b![ : ] b![ :: ] b![ , ] b![ / ] b![ /= ] b![ . ] b![ .. ] b![ ... ] b![ ..= ]                                                            /*
b![•:•]                                                                           ExpressionStatement{!semi}, MacroInvocation
  [•:•]                                                                           MacroInvocation.segments{dk: "[]"}
    :                                                                             PunctuationToken{tk: ":"}
        b![•::•]                                                                  ExpressionStatement{!semi}, MacroInvocation
          [•::•]                                                                  MacroInvocation.segments{dk: "[]"}
            ::                                                                    PunctuationToken{tk: "::"}
                 b![•,•]                                                          ExpressionStatement{!semi}, MacroInvocation
                   [•,•]                                                          MacroInvocation.segments{dk: "[]"}
                     ,                                                            PunctuationToken{tk: ","}
                         b![•/•]                                                  ExpressionStatement{!semi}, MacroInvocation
                           [•/•]                                                  MacroInvocation.segments{dk: "[]"}
                             /                                                    PunctuationToken{tk: "/"}
                                 b![•/=•]                                         ExpressionStatement{!semi}, MacroInvocation
                                   [•/=•]                                         MacroInvocation.segments{dk: "[]"}
                                     /=                                           PunctuationToken{tk: "/="}
                                          b![•.•]                                 ExpressionStatement{!semi}, MacroInvocation
                                            [•.•]                                 MacroInvocation.segments{dk: "[]"}
                                              .                                   PunctuationToken{tk: "."}
                                                  b![•..•]                        ExpressionStatement{!semi}, MacroInvocation
                                                    [•..•]                        MacroInvocation.segments{dk: "[]"}
                                                      ..                          PunctuationToken{tk: ".."}
                                                           b![•...•]              ExpressionStatement{!semi}, MacroInvocation
                                                             [•...•]              MacroInvocation.segments{dk: "[]"}
                                                               ...                PunctuationToken{tk: "..."}
                                                                     b![•..=•]    ExpressionStatement{!semi}, MacroInvocation
                                                                       [•..=•]    MacroInvocation.segments{dk: "[]"}
                                                                         ..=      PunctuationToken{tk: "..="}                             */
b![ = ] b![ == ] b![ >= ] b![ > ] b![ <= ] b![ < ] b![ *= ] b![ != ] b![ | ]                                                              /*
b![•=•]                                                                         ExpressionStatement{!semi}, MacroInvocation
  [•=•]                                                                         MacroInvocation.segments{dk: "[]"}
    =                                                                           PunctuationToken{tk: "="}
        b![•==•]                                                                ExpressionStatement{!semi}, MacroInvocation
          [•==•]                                                                MacroInvocation.segments{dk: "[]"}
            ==                                                                  PunctuationToken{tk: "=="}
                 b![•>=•]                                                       ExpressionStatement{!semi}, MacroInvocation
                   [•>=•]                                                       MacroInvocation.segments{dk: "[]"}
                     >=                                                         PunctuationToken{tk: ">="}
                          b![•>•]                                               ExpressionStatement{!semi}, MacroInvocation
                            [•>•]                                               MacroInvocation.segments{dk: "[]"}
                              >                                                 PunctuationToken{tk: ">"}
                                  b![•<=•]                                      ExpressionStatement{!semi}, MacroInvocation
                                    [•<=•]                                      MacroInvocation.segments{dk: "[]"}
                                      <=                                        PunctuationToken{tk: "<="}
                                           b![•<•]                              ExpressionStatement{!semi}, MacroInvocation
                                             [•<•]                              MacroInvocation.segments{dk: "[]"}
                                               <                                PunctuationToken{tk: "<"}
                                                   b![•*=•]                     ExpressionStatement{!semi}, MacroInvocation
                                                     [•*=•]                     MacroInvocation.segments{dk: "[]"}
                                                       *=                       PunctuationToken{tk: "*="}
                                                            b![•!=•]            ExpressionStatement{!semi}, MacroInvocation
                                                              [•!=•]            MacroInvocation.segments{dk: "[]"}
                                                                !=              PunctuationToken{tk: "!="}
                                                                     b![•|•]    ExpressionStatement{!semi}, MacroInvocation
                                                                       [•|•]    MacroInvocation.segments{dk: "[]"}
                                                                         |      PunctuationToken{tk: "|"}                                 */
b![ |= ] b![ || ] b![ # ] b![ ? ] b![ -> ] b![ <- ] b![ % ] b![ %= ] b![ => ]                                                             /*
b![•|=•]                                                                         ExpressionStatement{!semi}, MacroInvocation
  [•|=•]                                                                         MacroInvocation.segments{dk: "[]"}
    |=                                                                           PunctuationToken{tk: "|="}
         b![•||•]                                                                ExpressionStatement{!semi}, MacroInvocation
           [•||•]                                                                MacroInvocation.segments{dk: "[]"}
             ||                                                                  PunctuationToken{tk: "||"}
                  b![•#•]                                                        ExpressionStatement{!semi}, MacroInvocation
                    [•#•]                                                        MacroInvocation.segments{dk: "[]"}
                      #                                                          PunctuationToken{tk: "#"}
                          b![•?•]                                                ExpressionStatement{!semi}, MacroInvocation
                            [•?•]                                                MacroInvocation.segments{dk: "[]"}
                              ?                                                  PunctuationToken{tk: "?"}
                                  b![•->•]                                       ExpressionStatement{!semi}, MacroInvocation
                                    [•->•]                                       MacroInvocation.segments{dk: "[]"}
                                      ->                                         PunctuationToken{tk: "->"}
                                           b![•<-•]                              ExpressionStatement{!semi}, MacroInvocation
                                             [•<-•]                              MacroInvocation.segments{dk: "[]"}
                                               <                                 PunctuationToken{tk: "<"}
                                                -                                PunctuationToken{tk: "-"}
                                                    b![•%•]                      ExpressionStatement{!semi}, MacroInvocation
                                                      [•%•]                      MacroInvocation.segments{dk: "[]"}
                                                        %                        PunctuationToken{tk: "%"}
                                                            b![•%=•]             ExpressionStatement{!semi}, MacroInvocation
                                                              [•%=•]             MacroInvocation.segments{dk: "[]"}
                                                                %=               PunctuationToken{tk: "%="}
                                                                     b![•=>•]    ExpressionStatement{!semi}, MacroInvocation
                                                                       [•=>•]    MacroInvocation.segments{dk: "[]"}
                                                                         =>      PunctuationToken{tk: "=>"}                               */
b![ ; ] b![ << ] b![ <<= ] b![ >> ] b![ >>= ] b![ * ] b![ - ] b![ -= ] b![ ~ ]                                                            /*
b![•;•]                                                                           ExpressionStatement{!semi}, MacroInvocation
  [•;•]                                                                           MacroInvocation.segments{dk: "[]"}
    ;                                                                             PunctuationToken{tk: ";"}
        b![•<<•]                                                                  ExpressionStatement{!semi}, MacroInvocation
          [•<<•]                                                                  MacroInvocation.segments{dk: "[]"}
            <<                                                                    PunctuationToken{tk: "<<"}
                 b![•<<=•]                                                        ExpressionStatement{!semi}, MacroInvocation
                   [•<<=•]                                                        MacroInvocation.segments{dk: "[]"}
                     <<=                                                          PunctuationToken{tk: "<<="}
                           b![•>>•]                                               ExpressionStatement{!semi}, MacroInvocation
                             [•>>•]                                               MacroInvocation.segments{dk: "[]"}
                               >>                                                 PunctuationToken{tk: ">>"}
                                    b![•>>=•]                                     ExpressionStatement{!semi}, MacroInvocation
                                      [•>>=•]                                     MacroInvocation.segments{dk: "[]"}
                                        >>=                                       PunctuationToken{tk: ">>="}
                                              b![•*•]                             ExpressionStatement{!semi}, MacroInvocation
                                                [•*•]                             MacroInvocation.segments{dk: "[]"}
                                                  *                               PunctuationToken{tk: "*"}
                                                      b![•-•]                     ExpressionStatement{!semi}, MacroInvocation
                                                        [•-•]                     MacroInvocation.segments{dk: "[]"}
                                                          -                       PunctuationToken{tk: "-"}
                                                              b![•-=•]            ExpressionStatement{!semi}, MacroInvocation
                                                                [•-=•]            MacroInvocation.segments{dk: "[]"}
                                                                  -=              PunctuationToken{tk: "-="}
                                                                       b![•~•]    ExpressionStatement{!semi}, MacroInvocation
                                                                         [•~•]    MacroInvocation.segments{dk: "[]"}
                                                                           ~      PunctuationToken{tk: "~"}                               */

c!( + ) c!( += ) c!( & ) c!( && ) c!( &= ) c!( @ ) c!( ! ) c!( ^ ) c!( ^= )                                                               /*
c!(•+•)                                                                        ExpressionStatement{!semi}, MacroInvocation
  (•+•)                                                                        MacroInvocation.segments{dk: "()"}
    +                                                                          PunctuationToken{tk: "+"}
        c!(•+=•)                                                               ExpressionStatement{!semi}, MacroInvocation
          (•+=•)                                                               MacroInvocation.segments{dk: "()"}
            +=                                                                 PunctuationToken{tk: "+="}
                 c!(•&•)                                                       ExpressionStatement{!semi}, MacroInvocation
                   (•&•)                                                       MacroInvocation.segments{dk: "()"}
                     &                                                         PunctuationToken{tk: "&"}
                         c!(•&&•)                                              ExpressionStatement{!semi}, MacroInvocation
                           (•&&•)                                              MacroInvocation.segments{dk: "()"}
                             &&                                                PunctuationToken{tk: "&&"}
                                  c!(•&=•)                                     ExpressionStatement{!semi}, MacroInvocation
                                    (•&=•)                                     MacroInvocation.segments{dk: "()"}
                                      &=                                       PunctuationToken{tk: "&="}
                                           c!(•@•)                             ExpressionStatement{!semi}, MacroInvocation
                                             (•@•)                             MacroInvocation.segments{dk: "()"}
                                               @                               PunctuationToken{tk: "@"}
                                                   c!(•!•)                     ExpressionStatement{!semi}, MacroInvocation
                                                     (•!•)                     MacroInvocation.segments{dk: "()"}
                                                       !                       PunctuationToken{tk: "!"}
                                                           c!(•^•)             ExpressionStatement{!semi}, MacroInvocation
                                                             (•^•)             MacroInvocation.segments{dk: "()"}
                                                               ^               PunctuationToken{tk: "^"}
                                                                   c!(•^=•)    ExpressionStatement{!semi}, MacroInvocation
                                                                     (•^=•)    MacroInvocation.segments{dk: "()"}
                                                                       ^=      PunctuationToken{tk: "^="}                                 */
c!( : ) c!( :: ) c!( , ) c!( / ) c!( /= ) c!( . ) c!( .. ) c!( ... ) c!( ..= )                                                            /*
c!(•:•)                                                                           ExpressionStatement{!semi}, MacroInvocation
  (•:•)                                                                           MacroInvocation.segments{dk: "()"}
    :                                                                             PunctuationToken{tk: ":"}
        c!(•::•)                                                                  ExpressionStatement{!semi}, MacroInvocation
          (•::•)                                                                  MacroInvocation.segments{dk: "()"}
            ::                                                                    PunctuationToken{tk: "::"}
                 c!(•,•)                                                          ExpressionStatement{!semi}, MacroInvocation
                   (•,•)                                                          MacroInvocation.segments{dk: "()"}
                     ,                                                            PunctuationToken{tk: ","}
                         c!(•/•)                                                  ExpressionStatement{!semi}, MacroInvocation
                           (•/•)                                                  MacroInvocation.segments{dk: "()"}
                             /                                                    PunctuationToken{tk: "/"}
                                 c!(•/=•)                                         ExpressionStatement{!semi}, MacroInvocation
                                   (•/=•)                                         MacroInvocation.segments{dk: "()"}
                                     /=                                           PunctuationToken{tk: "/="}
                                          c!(•.•)                                 ExpressionStatement{!semi}, MacroInvocation
                                            (•.•)                                 MacroInvocation.segments{dk: "()"}
                                              .                                   PunctuationToken{tk: "."}
                                                  c!(•..•)                        ExpressionStatement{!semi}, MacroInvocation
                                                    (•..•)                        MacroInvocation.segments{dk: "()"}
                                                      ..                          PunctuationToken{tk: ".."}
                                                           c!(•...•)              ExpressionStatement{!semi}, MacroInvocation
                                                             (•...•)              MacroInvocation.segments{dk: "()"}
                                                               ...                PunctuationToken{tk: "..."}
                                                                     c!(•..=•)    ExpressionStatement{!semi}, MacroInvocation
                                                                       (•..=•)    MacroInvocation.segments{dk: "()"}
                                                                         ..=      PunctuationToken{tk: "..="}                             */
c!( = ) c!( == ) c!( >= ) c!( > ) c!( <= ) c!( < ) c!( *= ) c!( != ) c!( | )                                                              /*
c!(•=•)                                                                         ExpressionStatement{!semi}, MacroInvocation
  (•=•)                                                                         MacroInvocation.segments{dk: "()"}
    =                                                                           PunctuationToken{tk: "="}
        c!(•==•)                                                                ExpressionStatement{!semi}, MacroInvocation
          (•==•)                                                                MacroInvocation.segments{dk: "()"}
            ==                                                                  PunctuationToken{tk: "=="}
                 c!(•>=•)                                                       ExpressionStatement{!semi}, MacroInvocation
                   (•>=•)                                                       MacroInvocation.segments{dk: "()"}
                     >=                                                         PunctuationToken{tk: ">="}
                          c!(•>•)                                               ExpressionStatement{!semi}, MacroInvocation
                            (•>•)                                               MacroInvocation.segments{dk: "()"}
                              >                                                 PunctuationToken{tk: ">"}
                                  c!(•<=•)                                      ExpressionStatement{!semi}, MacroInvocation
                                    (•<=•)                                      MacroInvocation.segments{dk: "()"}
                                      <=                                        PunctuationToken{tk: "<="}
                                           c!(•<•)                              ExpressionStatement{!semi}, MacroInvocation
                                             (•<•)                              MacroInvocation.segments{dk: "()"}
                                               <                                PunctuationToken{tk: "<"}
                                                   c!(•*=•)                     ExpressionStatement{!semi}, MacroInvocation
                                                     (•*=•)                     MacroInvocation.segments{dk: "()"}
                                                       *=                       PunctuationToken{tk: "*="}
                                                            c!(•!=•)            ExpressionStatement{!semi}, MacroInvocation
                                                              (•!=•)            MacroInvocation.segments{dk: "()"}
                                                                !=              PunctuationToken{tk: "!="}
                                                                     c!(•|•)    ExpressionStatement{!semi}, MacroInvocation
                                                                       (•|•)    MacroInvocation.segments{dk: "()"}
                                                                         |      PunctuationToken{tk: "|"}                                 */
c!( |= ) c!( || ) c!( # ) c!( ? ) c!( -> ) c!( <- ) c!( % ) c!( %= ) c!( => )                                                             /*
c!(•|=•)                                                                         ExpressionStatement{!semi}, MacroInvocation
  (•|=•)                                                                         MacroInvocation.segments{dk: "()"}
    |=                                                                           PunctuationToken{tk: "|="}
         c!(•||•)                                                                ExpressionStatement{!semi}, MacroInvocation
           (•||•)                                                                MacroInvocation.segments{dk: "()"}
             ||                                                                  PunctuationToken{tk: "||"}
                  c!(•#•)                                                        ExpressionStatement{!semi}, MacroInvocation
                    (•#•)                                                        MacroInvocation.segments{dk: "()"}
                      #                                                          PunctuationToken{tk: "#"}
                          c!(•?•)                                                ExpressionStatement{!semi}, MacroInvocation
                            (•?•)                                                MacroInvocation.segments{dk: "()"}
                              ?                                                  PunctuationToken{tk: "?"}
                                  c!(•->•)                                       ExpressionStatement{!semi}, MacroInvocation
                                    (•->•)                                       MacroInvocation.segments{dk: "()"}
                                      ->                                         PunctuationToken{tk: "->"}
                                           c!(•<-•)                              ExpressionStatement{!semi}, MacroInvocation
                                             (•<-•)                              MacroInvocation.segments{dk: "()"}
                                               <                                 PunctuationToken{tk: "<"}
                                                -                                PunctuationToken{tk: "-"}
                                                    c!(•%•)                      ExpressionStatement{!semi}, MacroInvocation
                                                      (•%•)                      MacroInvocation.segments{dk: "()"}
                                                        %                        PunctuationToken{tk: "%"}
                                                            c!(•%=•)             ExpressionStatement{!semi}, MacroInvocation
                                                              (•%=•)             MacroInvocation.segments{dk: "()"}
                                                                %=               PunctuationToken{tk: "%="}
                                                                     c!(•=>•)    ExpressionStatement{!semi}, MacroInvocation
                                                                       (•=>•)    MacroInvocation.segments{dk: "()"}
                                                                         =>      PunctuationToken{tk: "=>"}                               */
c!( ; ) c!( << ) c!( <<= ) c!( >> ) c!( >>= ) c!( * ) c!( - ) c!( -= ) c!( ~ )                                                            /*
c!(•;•)                                                                           ExpressionStatement{!semi}, MacroInvocation
  (•;•)                                                                           MacroInvocation.segments{dk: "()"}
    ;                                                                             PunctuationToken{tk: ";"}
        c!(•<<•)                                                                  ExpressionStatement{!semi}, MacroInvocation
          (•<<•)                                                                  MacroInvocation.segments{dk: "()"}
            <<                                                                    PunctuationToken{tk: "<<"}
                 c!(•<<=•)                                                        ExpressionStatement{!semi}, MacroInvocation
                   (•<<=•)                                                        MacroInvocation.segments{dk: "()"}
                     <<=                                                          PunctuationToken{tk: "<<="}
                           c!(•>>•)                                               ExpressionStatement{!semi}, MacroInvocation
                             (•>>•)                                               MacroInvocation.segments{dk: "()"}
                               >>                                                 PunctuationToken{tk: ">>"}
                                    c!(•>>=•)                                     ExpressionStatement{!semi}, MacroInvocation
                                      (•>>=•)                                     MacroInvocation.segments{dk: "()"}
                                        >>=                                       PunctuationToken{tk: ">>="}
                                              c!(•*•)                             ExpressionStatement{!semi}, MacroInvocation
                                                (•*•)                             MacroInvocation.segments{dk: "()"}
                                                  *                               PunctuationToken{tk: "*"}
                                                      c!(•-•)                     ExpressionStatement{!semi}, MacroInvocation
                                                        (•-•)                     MacroInvocation.segments{dk: "()"}
                                                          -                       PunctuationToken{tk: "-"}
                                                              c!(•-=•)            ExpressionStatement{!semi}, MacroInvocation
                                                                (•-=•)            MacroInvocation.segments{dk: "()"}
                                                                  -=              PunctuationToken{tk: "-="}
                                                                       c!(•~•)    ExpressionStatement{!semi}, MacroInvocation
                                                                         (•~•)    MacroInvocation.segments{dk: "()"}
                                                                           ~      PunctuationToken{tk: "~"}                               */

macro_rules! x {                                                                                                                          /*
macro_rules!•x•{↲    <MacroRulesDeclaration>
               {↲    <MacroRulesDeclaration.rules{dk: "{}"}>                                                                              */
	($p:pat =>) => {};                                                                                                                    /*
	($p:pat•=>)•=>•{}    MacroRuleDeclaration
	($p:pat•=>)          MacroRuleDeclaration.match{dk: "()"}
	 $p:pat              MacroParameterDeclaration
	 $p                  McIdentifier
	        =>           PunctuationToken{tk: "=>"}
	               {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:pat ,) => {};                                                                                                                     /*
    ($p:pat•,)•=>•{}    MacroRuleDeclaration
    ($p:pat•,)          MacroRuleDeclaration.match{dk: "()"}
     $p:pat             MacroParameterDeclaration
     $p                 McIdentifier
            ,           PunctuationToken{tk: ","}
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($p:pat =) => {};                                                                                                                     /*
    ($p:pat•=)•=>•{}    MacroRuleDeclaration
    ($p:pat•=)          MacroRuleDeclaration.match{dk: "()"}
     $p:pat             MacroParameterDeclaration
     $p                 McIdentifier
            =           PunctuationToken{tk: "="}
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($p:pat |) => {};                                                                                                                     /*
    ($p:pat•|)•=>•{}    MacroRuleDeclaration
    ($p:pat•|)          MacroRuleDeclaration.match{dk: "()"}
     $p:pat             MacroParameterDeclaration
     $p                 McIdentifier
            |           PunctuationToken{tk: "|"}
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($p:pat if) => {};                                                                                                                    /*
    ($p:pat•if)•=>•{}    MacroRuleDeclaration
    ($p:pat•if)          MacroRuleDeclaration.match{dk: "()"}
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:pat in) => {};                                                                                                                    /*
    ($p:pat•in)•=>•{}    MacroRuleDeclaration
    ($p:pat•in)          MacroRuleDeclaration.match{dk: "()"}
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
	($e:expr =>) => {};                                                                                                                   /*
	($e:expr•=>)•=>•{}    MacroRuleDeclaration
	($e:expr•=>)          MacroRuleDeclaration.match{dk: "()"}
	 $e:expr              MacroParameterDeclaration
	 $e                   McIdentifier
	         =>           PunctuationToken{tk: "=>"}
	                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($e:expr ,) => {};                                                                                                                    /*
    ($e:expr•,)•=>•{}    MacroRuleDeclaration
    ($e:expr•,)          MacroRuleDeclaration.match{dk: "()"}
     $e:expr             MacroParameterDeclaration
     $e                  McIdentifier
             ,           PunctuationToken{tk: ","}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($e:expr ;) => {};                                                                                                                    /*
    ($e:expr•;)•=>•{}    MacroRuleDeclaration
    ($e:expr•;)          MacroRuleDeclaration.match{dk: "()"}
     $e:expr             MacroParameterDeclaration
     $e                  McIdentifier
             ;           PunctuationToken{tk: ";"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
	($t:ty {}) => {};                                                                                                                     /*
	($t:ty•{})•=>•{}    MacroRuleDeclaration
	($t:ty•{})          MacroRuleDeclaration.match{dk: "()"}
	 $t:ty              MacroParameterDeclaration
	 $t                 McIdentifier
	       {}           DelimGroup
	              {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:ty ,) => {};                                                                                                                      /*
    ($t:ty•,)•=>•{}    MacroRuleDeclaration
    ($t:ty•,)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           ,           PunctuationToken{tk: ","}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty =>) => {};                                                                                                                     /*
    ($t:ty•=>)•=>•{}    MacroRuleDeclaration
    ($t:ty•=>)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           =>           PunctuationToken{tk: "=>"}
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:ty :) => {};                                                                                                                      /*
    ($t:ty•:)•=>•{}    MacroRuleDeclaration
    ($t:ty•:)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           :           PunctuationToken{tk: ":"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty =) => {};                                                                                                                      /*
    ($t:ty•=)•=>•{}    MacroRuleDeclaration
    ($t:ty•=)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           =           PunctuationToken{tk: "="}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty >) => {};                                                                                                                      /*
    ($t:ty•>)•=>•{}    MacroRuleDeclaration
    ($t:ty•>)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           >           PunctuationToken{tk: ">"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty ;) => {};                                                                                                                      /*
    ($t:ty•;)•=>•{}    MacroRuleDeclaration
    ($t:ty•;)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           ;           PunctuationToken{tk: ";"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty |) => {};                                                                                                                      /*
    ($t:ty•|)•=>•{}    MacroRuleDeclaration
    ($t:ty•|)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty             MacroParameterDeclaration
     $t                McIdentifier
           |           PunctuationToken{tk: "|"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:ty as) => {};                                                                                                                     /*
    ($t:ty•as)•=>•{}    MacroRuleDeclaration
    ($t:ty•as)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:ty where) => {};                                                                                                                  /*
    ($t:ty•where)•=>•{}    MacroRuleDeclaration
    ($t:ty•where)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty                 MacroParameterDeclaration
     $t                    McIdentifier
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($t:ty []) => {};                                                                                                                     /*
    ($t:ty•[])•=>•{}    MacroRuleDeclaration
    ($t:ty•[])          MacroRuleDeclaration.match{dk: "()"}
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           []           DelimGroup
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:ty $b:block) => {};                                                                                                               /*
    ($t:ty•$b:block)•=>•{}    MacroRuleDeclaration
    ($t:ty•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $t:ty                    MacroParameterDeclaration
     $t                       McIdentifier
           $b:block           MacroParameterDeclaration
           $b                 McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
	($s:stmt =>) => {};                                                                                                                   /*
	($s:stmt•=>)•=>•{}    MacroRuleDeclaration
	($s:stmt•=>)          MacroRuleDeclaration.match{dk: "()"}
	 $s:stmt              MacroParameterDeclaration
	 $s                   McIdentifier
	         =>           PunctuationToken{tk: "=>"}
	                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($s:stmt ,) => {};                                                                                                                    /*
    ($s:stmt•,)•=>•{}    MacroRuleDeclaration
    ($s:stmt•,)          MacroRuleDeclaration.match{dk: "()"}
     $s:stmt             MacroParameterDeclaration
     $s                  McIdentifier
             ,           PunctuationToken{tk: ","}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($s:stmt ;) => {};                                                                                                                    /*
    ($s:stmt•;)•=>•{}    MacroRuleDeclaration
    ($s:stmt•;)          MacroRuleDeclaration.match{dk: "()"}
     $s:stmt             MacroParameterDeclaration
     $s                  McIdentifier
             ;           PunctuationToken{tk: ";"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
	($p:path {}) => {};                                                                                                                   /*
	($p:path•{})•=>•{}    MacroRuleDeclaration
	($p:path•{})          MacroRuleDeclaration.match{dk: "()"}
	 $p:path              MacroParameterDeclaration
	 $p                   McIdentifier
	         {}           DelimGroup
	                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($p:path ,) => {};                                                                                                                    /*
    ($p:path•,)•=>•{}    MacroRuleDeclaration
    ($p:path•,)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             ,           PunctuationToken{tk: ","}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path =>) => {};                                                                                                                   /*
    ($p:path•=>)•=>•{}    MacroRuleDeclaration
    ($p:path•=>)          MacroRuleDeclaration.match{dk: "()"}
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             =>           PunctuationToken{tk: "=>"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($p:path :) => {};                                                                                                                    /*
    ($p:path•:)•=>•{}    MacroRuleDeclaration
    ($p:path•:)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             :           PunctuationToken{tk: ":"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path =) => {};                                                                                                                    /*
    ($p:path•=)•=>•{}    MacroRuleDeclaration
    ($p:path•=)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             =           PunctuationToken{tk: "="}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path >) => {};                                                                                                                    /*
    ($p:path•>)•=>•{}    MacroRuleDeclaration
    ($p:path•>)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             >           PunctuationToken{tk: ">"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path ;) => {};                                                                                                                    /*
    ($p:path•;)•=>•{}    MacroRuleDeclaration
    ($p:path•;)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             ;           PunctuationToken{tk: ";"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path |) => {};                                                                                                                    /*
    ($p:path•|)•=>•{}    MacroRuleDeclaration
    ($p:path•|)          MacroRuleDeclaration.match{dk: "()"}
     $p:path             MacroParameterDeclaration
     $p                  McIdentifier
             |           PunctuationToken{tk: "|"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($p:path as) => {};                                                                                                                   /*
    ($p:path•as)•=>•{}    MacroRuleDeclaration
    ($p:path•as)          MacroRuleDeclaration.match{dk: "()"}
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($p:path where) => {};                                                                                                                /*
    ($p:path•where)•=>•{}    MacroRuleDeclaration
    ($p:path•where)          MacroRuleDeclaration.match{dk: "()"}
     $p:path                 MacroParameterDeclaration
     $p                      McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($p:path []) => {};                                                                                                                   /*
    ($p:path•[])•=>•{}    MacroRuleDeclaration
    ($p:path•[])          MacroRuleDeclaration.match{dk: "()"}
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             []           DelimGroup
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($p:path $b:block) => {};                                                                                                             /*
    ($p:path•$b:block)•=>•{}    MacroRuleDeclaration
    ($p:path•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $p:path                    MacroParameterDeclaration
     $p                         McIdentifier
             $b:block           MacroParameterDeclaration
             $b                 McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
	($b:block ()) => {};                                                                                                                  /*
	($b:block•())•=>•{}    MacroRuleDeclaration
	($b:block•())          MacroRuleDeclaration.match{dk: "()"}
	 $b:block              MacroParameterDeclaration
	 $b                    McIdentifier
	          ()           DelimGroup
	                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($b:block []) => {};                                                                                                                  /*
    ($b:block•[])•=>•{}    MacroRuleDeclaration
    ($b:block•[])          MacroRuleDeclaration.match{dk: "()"}
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              []           DelimGroup
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($b:block {}) => {};                                                                                                                  /*
    ($b:block•{})•=>•{}    MacroRuleDeclaration
    ($b:block•{})          MacroRuleDeclaration.match{dk: "()"}
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              {}           DelimGroup
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($b:block ,) => {};                                                                                                                   /*
    ($b:block•,)•=>•{}    MacroRuleDeclaration
    ($b:block•,)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              ,           PunctuationToken{tk: ","}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block =>) => {};                                                                                                                  /*
    ($b:block•=>)•=>•{}    MacroRuleDeclaration
    ($b:block•=>)          MacroRuleDeclaration.match{dk: "()"}
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              =>           PunctuationToken{tk: "=>"}
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($b:block :) => {};                                                                                                                   /*
    ($b:block•:)•=>•{}    MacroRuleDeclaration
    ($b:block•:)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              :           PunctuationToken{tk: ":"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block =) => {};                                                                                                                   /*
    ($b:block•=)•=>•{}    MacroRuleDeclaration
    ($b:block•=)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              =           PunctuationToken{tk: "="}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block >) => {};                                                                                                                   /*
    ($b:block•>)•=>•{}    MacroRuleDeclaration
    ($b:block•>)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              >           PunctuationToken{tk: ">"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block ;) => {};                                                                                                                   /*
    ($b:block•;)•=>•{}    MacroRuleDeclaration
    ($b:block•;)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              ;           PunctuationToken{tk: ";"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block |) => {};                                                                                                                   /*
    ($b:block•|)•=>•{}    MacroRuleDeclaration
    ($b:block•|)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              |           PunctuationToken{tk: "|"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block +) => {};                                                                                                                   /*
    ($b:block•+)•=>•{}    MacroRuleDeclaration
    ($b:block•+)          MacroRuleDeclaration.match{dk: "()"}
     $b:block             MacroParameterDeclaration
     $b                   McIdentifier
              +           PunctuationToken{tk: "+"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($b:block ident) => {};                                                                                                               /*
    ($b:block•ident)•=>•{}    MacroRuleDeclaration
    ($b:block•ident)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                 MacroParameterDeclaration
     $b                       McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($b:block $p:pat) => {};                                                                                                              /*
    ($b:block•$p:pat)•=>•{}    MacroRuleDeclaration
    ($b:block•$p:pat)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                  MacroParameterDeclaration
     $b                        McIdentifier
              $p:pat           MacroParameterDeclaration
              $p               McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($b:block $e:expr) => {};                                                                                                             /*
    ($b:block•$e:expr)•=>•{}    MacroRuleDeclaration
    ($b:block•$e:expr)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $e:expr           MacroParameterDeclaration
              $e                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($b:block $t:ty) => {};                                                                                                               /*
    ($b:block•$t:ty)•=>•{}    MacroRuleDeclaration
    ($b:block•$t:ty)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                 MacroParameterDeclaration
     $b                       McIdentifier
              $t:ty           MacroParameterDeclaration
              $t              McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($b:block $s:stmt) => {};                                                                                                             /*
    ($b:block•$s:stmt)•=>•{}    MacroRuleDeclaration
    ($b:block•$s:stmt)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $s:stmt           MacroParameterDeclaration
              $s                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($b:block $p:path) => {};                                                                                                             /*
    ($b:block•$p:path)•=>•{}    MacroRuleDeclaration
    ($b:block•$p:path)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $p:path           MacroParameterDeclaration
              $p                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($b:block $c:block) => {};                                                                                                            /*
    ($b:block•$c:block)•=>•{}    MacroRuleDeclaration
    ($b:block•$c:block)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $c:block           MacroParameterDeclaration
              $c                 McIdentifier
                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                 */
    ($b:block $i:ident) => {};                                                                                                            /*
    ($b:block•$i:ident)•=>•{}    MacroRuleDeclaration
    ($b:block•$i:ident)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $i:ident           MacroParameterDeclaration
              $i                 McIdentifier
                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                 */
    ($b:block $t:tt) => {};                                                                                                               /*
    ($b:block•$t:tt)•=>•{}    MacroRuleDeclaration
    ($b:block•$t:tt)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                 MacroParameterDeclaration
     $b                       McIdentifier
              $t:tt           MacroParameterDeclaration
              $t              McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($b:block $i:item) => {};                                                                                                             /*
    ($b:block•$i:item)•=>•{}    MacroRuleDeclaration
    ($b:block•$i:item)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $i:item           MacroParameterDeclaration
              $i                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($b:block $m:meta) => {};                                                                                                             /*
    ($b:block•$m:meta)•=>•{}    MacroRuleDeclaration
    ($b:block•$m:meta)          MacroRuleDeclaration.match{dk: "()"}
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $m:meta           MacroParameterDeclaration
              $m                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
	($i:ident ()) => {};                                                                                                                  /*
	($i:ident•())•=>•{}    MacroRuleDeclaration
	($i:ident•())          MacroRuleDeclaration.match{dk: "()"}
	 $i:ident              MacroParameterDeclaration
	 $i                    McIdentifier
	          ()           DelimGroup
	                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($i:ident []) => {};                                                                                                                  /*
    ($i:ident•[])•=>•{}    MacroRuleDeclaration
    ($i:ident•[])          MacroRuleDeclaration.match{dk: "()"}
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              []           DelimGroup
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($i:ident {}) => {};                                                                                                                  /*
    ($i:ident•{})•=>•{}    MacroRuleDeclaration
    ($i:ident•{})          MacroRuleDeclaration.match{dk: "()"}
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              {}           DelimGroup
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($i:ident ,) => {};                                                                                                                   /*
    ($i:ident•,)•=>•{}    MacroRuleDeclaration
    ($i:ident•,)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              ,           PunctuationToken{tk: ","}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident =>) => {};                                                                                                                  /*
    ($i:ident•=>)•=>•{}    MacroRuleDeclaration
    ($i:ident•=>)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              =>           PunctuationToken{tk: "=>"}
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($i:ident :) => {};                                                                                                                   /*
    ($i:ident•:)•=>•{}    MacroRuleDeclaration
    ($i:ident•:)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              :           PunctuationToken{tk: ":"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident =) => {};                                                                                                                   /*
    ($i:ident•=)•=>•{}    MacroRuleDeclaration
    ($i:ident•=)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              =           PunctuationToken{tk: "="}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident >) => {};                                                                                                                   /*
    ($i:ident•>)•=>•{}    MacroRuleDeclaration
    ($i:ident•>)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              >           PunctuationToken{tk: ">"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident ;) => {};                                                                                                                   /*
    ($i:ident•;)•=>•{}    MacroRuleDeclaration
    ($i:ident•;)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              ;           PunctuationToken{tk: ";"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident |) => {};                                                                                                                   /*
    ($i:ident•|)•=>•{}    MacroRuleDeclaration
    ($i:ident•|)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              |           PunctuationToken{tk: "|"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident +) => {};                                                                                                                   /*
    ($i:ident•+)•=>•{}    MacroRuleDeclaration
    ($i:ident•+)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident             MacroParameterDeclaration
     $i                   McIdentifier
              +           PunctuationToken{tk: "+"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:ident ident) => {};                                                                                                               /*
    ($i:ident•ident)•=>•{}    MacroRuleDeclaration
    ($i:ident•ident)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                 MacroParameterDeclaration
     $i                       McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($i:ident $p:pat) => {};                                                                                                              /*
    ($i:ident•$p:pat)•=>•{}    MacroRuleDeclaration
    ($i:ident•$p:pat)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                  MacroParameterDeclaration
     $i                        McIdentifier
              $p:pat           MacroParameterDeclaration
              $p               McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($i:ident $e:expr) => {};                                                                                                             /*
    ($i:ident•$e:expr)•=>•{}    MacroRuleDeclaration
    ($i:ident•$e:expr)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $e:expr           MacroParameterDeclaration
              $e                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:ident $t:ty) => {};                                                                                                               /*
    ($i:ident•$t:ty)•=>•{}    MacroRuleDeclaration
    ($i:ident•$t:ty)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                 MacroParameterDeclaration
     $i                       McIdentifier
              $t:ty           MacroParameterDeclaration
              $t              McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($i:ident $s:stmt) => {};                                                                                                             /*
    ($i:ident•$s:stmt)•=>•{}    MacroRuleDeclaration
    ($i:ident•$s:stmt)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $s:stmt           MacroParameterDeclaration
              $s                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:ident $p:path) => {};                                                                                                             /*
    ($i:ident•$p:path)•=>•{}    MacroRuleDeclaration
    ($i:ident•$p:path)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $p:path           MacroParameterDeclaration
              $p                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:ident $b:block) => {};                                                                                                            /*
    ($i:ident•$b:block)•=>•{}    MacroRuleDeclaration
    ($i:ident•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $b:block           MacroParameterDeclaration
              $b                 McIdentifier
                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                 */
    ($i:ident $j:ident) => {};                                                                                                            /*
    ($i:ident•$j:ident)•=>•{}    MacroRuleDeclaration
    ($i:ident•$j:ident)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $j:ident           MacroParameterDeclaration
              $j                 McIdentifier
                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                 */
    ($i:ident $t:tt) => {};                                                                                                               /*
    ($i:ident•$t:tt)•=>•{}    MacroRuleDeclaration
    ($i:ident•$t:tt)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                 MacroParameterDeclaration
     $i                       McIdentifier
              $t:tt           MacroParameterDeclaration
              $t              McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($i:ident $j:item) => {};                                                                                                             /*
    ($i:ident•$j:item)•=>•{}    MacroRuleDeclaration
    ($i:ident•$j:item)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $j:item           MacroParameterDeclaration
              $j                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:ident $m:meta) => {};                                                                                                             /*
    ($i:ident•$m:meta)•=>•{}    MacroRuleDeclaration
    ($i:ident•$m:meta)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $m:meta           MacroParameterDeclaration
              $m                McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
	($t:tt ()) => {};                                                                                                                     /*
	($t:tt•())•=>•{}    MacroRuleDeclaration
	($t:tt•())          MacroRuleDeclaration.match{dk: "()"}
	 $t:tt              MacroParameterDeclaration
	 $t                 McIdentifier
	       ()           DelimGroup
	              {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:tt []) => {};                                                                                                                     /*
    ($t:tt•[])•=>•{}    MacroRuleDeclaration
    ($t:tt•[])          MacroRuleDeclaration.match{dk: "()"}
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           []           DelimGroup
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:tt {}) => {};                                                                                                                     /*
    ($t:tt•{})•=>•{}    MacroRuleDeclaration
    ($t:tt•{})          MacroRuleDeclaration.match{dk: "()"}
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           {}           DelimGroup
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:tt ,) => {};                                                                                                                      /*
    ($t:tt•,)•=>•{}    MacroRuleDeclaration
    ($t:tt•,)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           ,           PunctuationToken{tk: ","}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt =>) => {};                                                                                                                     /*
    ($t:tt•=>)•=>•{}    MacroRuleDeclaration
    ($t:tt•=>)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           =>           PunctuationToken{tk: "=>"}
                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                          */
    ($t:tt :) => {};                                                                                                                      /*
    ($t:tt•:)•=>•{}    MacroRuleDeclaration
    ($t:tt•:)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           :           PunctuationToken{tk: ":"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt =) => {};                                                                                                                      /*
    ($t:tt•=)•=>•{}    MacroRuleDeclaration
    ($t:tt•=)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           =           PunctuationToken{tk: "="}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt >) => {};                                                                                                                      /*
    ($t:tt•>)•=>•{}    MacroRuleDeclaration
    ($t:tt•>)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           >           PunctuationToken{tk: ">"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt ;) => {};                                                                                                                      /*
    ($t:tt•;)•=>•{}    MacroRuleDeclaration
    ($t:tt•;)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           ;           PunctuationToken{tk: ";"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt |) => {};                                                                                                                      /*
    ($t:tt•|)•=>•{}    MacroRuleDeclaration
    ($t:tt•|)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           |           PunctuationToken{tk: "|"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt +) => {};                                                                                                                      /*
    ($t:tt•+)•=>•{}    MacroRuleDeclaration
    ($t:tt•+)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt             MacroParameterDeclaration
     $t                McIdentifier
           +           PunctuationToken{tk: "+"}
                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                           */
    ($t:tt ident) => {};                                                                                                                  /*
    ($t:tt•ident)•=>•{}    MacroRuleDeclaration
    ($t:tt•ident)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                 MacroParameterDeclaration
     $t                    McIdentifier
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($t:tt $p:pat) => {};                                                                                                                 /*
    ($t:tt•$p:pat)•=>•{}    MacroRuleDeclaration
    ($t:tt•$p:pat)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                  MacroParameterDeclaration
     $t                     McIdentifier
           $p:pat           MacroParameterDeclaration
           $p               McIdentifier
                      {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                      */
    ($t:tt $e:expr) => {};                                                                                                                /*
    ($t:tt•$e:expr)•=>•{}    MacroRuleDeclaration
    ($t:tt•$e:expr)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $e:expr           MacroParameterDeclaration
           $e                McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($t:tt $v:ty) => {};                                                                                                                  /*
    ($t:tt•$v:ty)•=>•{}    MacroRuleDeclaration
    ($t:tt•$v:ty)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                 MacroParameterDeclaration
     $t                    McIdentifier
           $v:ty           MacroParameterDeclaration
           $v              McIdentifier
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($t:tt $s:stmt) => {};                                                                                                                /*
    ($t:tt•$s:stmt)•=>•{}    MacroRuleDeclaration
    ($t:tt•$s:stmt)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $s:stmt           MacroParameterDeclaration
           $s                McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($t:tt $p:path) => {};                                                                                                                /*
    ($t:tt•$p:path)•=>•{}    MacroRuleDeclaration
    ($t:tt•$p:path)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $p:path           MacroParameterDeclaration
           $p                McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($t:tt $b:block) => {};                                                                                                               /*
    ($t:tt•$b:block)•=>•{}    MacroRuleDeclaration
    ($t:tt•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $b:block           MacroParameterDeclaration
           $b                 McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($t:tt $i:ident) => {};                                                                                                               /*
    ($t:tt•$i:ident)•=>•{}    MacroRuleDeclaration
    ($t:tt•$i:ident)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $i:ident           MacroParameterDeclaration
           $i                 McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($t:tt $v:tt) => {};                                                                                                                  /*
    ($t:tt•$v:tt)•=>•{}    MacroRuleDeclaration
    ($t:tt•$v:tt)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                 MacroParameterDeclaration
     $t                    McIdentifier
           $v:tt           MacroParameterDeclaration
           $v              McIdentifier
                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                       */
    ($t:tt $i:item) => {};                                                                                                                /*
    ($t:tt•$i:item)•=>•{}    MacroRuleDeclaration
    ($t:tt•$i:item)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $i:item           MacroParameterDeclaration
           $i                McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($t:tt $m:meta) => {};                                                                                                                /*
    ($t:tt•$m:meta)•=>•{}    MacroRuleDeclaration
    ($t:tt•$m:meta)          MacroRuleDeclaration.match{dk: "()"}
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $m:meta           MacroParameterDeclaration
           $m                McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
	($i:item ()) => {};                                                                                                                   /*
	($i:item•())•=>•{}    MacroRuleDeclaration
	($i:item•())          MacroRuleDeclaration.match{dk: "()"}
	 $i:item              MacroParameterDeclaration
	 $i                   McIdentifier
	         ()           DelimGroup
	                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:item []) => {};                                                                                                                   /*
    ($i:item•[])•=>•{}    MacroRuleDeclaration
    ($i:item•[])          MacroRuleDeclaration.match{dk: "()"}
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             []           DelimGroup
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:item {}) => {};                                                                                                                   /*
    ($i:item•{})•=>•{}    MacroRuleDeclaration
    ($i:item•{})          MacroRuleDeclaration.match{dk: "()"}
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             {}           DelimGroup
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:item ,) => {};                                                                                                                    /*
    ($i:item•,)•=>•{}    MacroRuleDeclaration
    ($i:item•,)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             ,           PunctuationToken{tk: ","}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item =>) => {};                                                                                                                   /*
    ($i:item•=>)•=>•{}    MacroRuleDeclaration
    ($i:item•=>)          MacroRuleDeclaration.match{dk: "()"}
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             =>           PunctuationToken{tk: "=>"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($i:item :) => {};                                                                                                                    /*
    ($i:item•:)•=>•{}    MacroRuleDeclaration
    ($i:item•:)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             :           PunctuationToken{tk: ":"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item =) => {};                                                                                                                    /*
    ($i:item•=)•=>•{}    MacroRuleDeclaration
    ($i:item•=)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             =           PunctuationToken{tk: "="}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item >) => {};                                                                                                                    /*
    ($i:item•>)•=>•{}    MacroRuleDeclaration
    ($i:item•>)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             >           PunctuationToken{tk: ">"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item ;) => {};                                                                                                                    /*
    ($i:item•;)•=>•{}    MacroRuleDeclaration
    ($i:item•;)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             ;           PunctuationToken{tk: ";"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item |) => {};                                                                                                                    /*
    ($i:item•|)•=>•{}    MacroRuleDeclaration
    ($i:item•|)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             |           PunctuationToken{tk: "|"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item +) => {};                                                                                                                    /*
    ($i:item•+)•=>•{}    MacroRuleDeclaration
    ($i:item•+)          MacroRuleDeclaration.match{dk: "()"}
     $i:item             MacroParameterDeclaration
     $i                  McIdentifier
             +           PunctuationToken{tk: "+"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($i:item ident) => {};                                                                                                                /*
    ($i:item•ident)•=>•{}    MacroRuleDeclaration
    ($i:item•ident)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                 MacroParameterDeclaration
     $i                      McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($i:item $p:pat) => {};                                                                                                               /*
    ($i:item•$p:pat)•=>•{}    MacroRuleDeclaration
    ($i:item•$p:pat)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                  MacroParameterDeclaration
     $i                       McIdentifier
             $p:pat           MacroParameterDeclaration
             $p               McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($i:item $e:expr) => {};                                                                                                              /*
    ($i:item•$e:expr)•=>•{}    MacroRuleDeclaration
    ($i:item•$e:expr)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $e:expr           MacroParameterDeclaration
             $e                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($i:item $t:ty) => {};                                                                                                                /*
    ($i:item•$t:ty)•=>•{}    MacroRuleDeclaration
    ($i:item•$t:ty)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                 MacroParameterDeclaration
     $i                      McIdentifier
             $t:ty           MacroParameterDeclaration
             $t              McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($i:item $s:stmt) => {};                                                                                                              /*
    ($i:item•$s:stmt)•=>•{}    MacroRuleDeclaration
    ($i:item•$s:stmt)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $s:stmt           MacroParameterDeclaration
             $s                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($i:item $p:path) => {};                                                                                                              /*
    ($i:item•$p:path)•=>•{}    MacroRuleDeclaration
    ($i:item•$p:path)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $p:path           MacroParameterDeclaration
             $p                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($i:item $b:block) => {};                                                                                                             /*
    ($i:item•$b:block)•=>•{}    MacroRuleDeclaration
    ($i:item•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $b:block           MacroParameterDeclaration
             $b                 McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:item $j:ident) => {};                                                                                                             /*
    ($i:item•$j:ident)•=>•{}    MacroRuleDeclaration
    ($i:item•$j:ident)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $j:ident           MacroParameterDeclaration
             $j                 McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($i:item $t:tt) => {};                                                                                                                /*
    ($i:item•$t:tt)•=>•{}    MacroRuleDeclaration
    ($i:item•$t:tt)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                 MacroParameterDeclaration
     $i                      McIdentifier
             $t:tt           MacroParameterDeclaration
             $t              McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($i:item $j:item) => {};                                                                                                              /*
    ($i:item•$j:item)•=>•{}    MacroRuleDeclaration
    ($i:item•$j:item)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $j:item           MacroParameterDeclaration
             $j                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($i:item $m:meta) => {};                                                                                                              /*
    ($i:item•$m:meta)•=>•{}    MacroRuleDeclaration
    ($i:item•$m:meta)          MacroRuleDeclaration.match{dk: "()"}
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $m:meta           MacroParameterDeclaration
             $m                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
	($m:meta ()) => {};                                                                                                                   /*
	($m:meta•())•=>•{}    MacroRuleDeclaration
	($m:meta•())          MacroRuleDeclaration.match{dk: "()"}
	 $m:meta              MacroParameterDeclaration
	 $m                   McIdentifier
	         ()           DelimGroup
	                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($m:meta []) => {};                                                                                                                   /*
    ($m:meta•[])•=>•{}    MacroRuleDeclaration
    ($m:meta•[])          MacroRuleDeclaration.match{dk: "()"}
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             []           DelimGroup
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($m:meta {}) => {};                                                                                                                   /*
    ($m:meta•{})•=>•{}    MacroRuleDeclaration
    ($m:meta•{})          MacroRuleDeclaration.match{dk: "()"}
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             {}           DelimGroup
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($m:meta ,) => {};                                                                                                                    /*
    ($m:meta•,)•=>•{}    MacroRuleDeclaration
    ($m:meta•,)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             ,           PunctuationToken{tk: ","}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta =>) => {};                                                                                                                   /*
    ($m:meta•=>)•=>•{}    MacroRuleDeclaration
    ($m:meta•=>)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             =>           PunctuationToken{tk: "=>"}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    ($m:meta :) => {};                                                                                                                    /*
    ($m:meta•:)•=>•{}    MacroRuleDeclaration
    ($m:meta•:)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             :           PunctuationToken{tk: ":"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta =) => {};                                                                                                                    /*
    ($m:meta•=)•=>•{}    MacroRuleDeclaration
    ($m:meta•=)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             =           PunctuationToken{tk: "="}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta >) => {};                                                                                                                    /*
    ($m:meta•>)•=>•{}    MacroRuleDeclaration
    ($m:meta•>)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             >           PunctuationToken{tk: ">"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta ;) => {};                                                                                                                    /*
    ($m:meta•;)•=>•{}    MacroRuleDeclaration
    ($m:meta•;)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             ;           PunctuationToken{tk: ";"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta |) => {};                                                                                                                    /*
    ($m:meta•|)•=>•{}    MacroRuleDeclaration
    ($m:meta•|)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             |           PunctuationToken{tk: "|"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta +) => {};                                                                                                                    /*
    ($m:meta•+)•=>•{}    MacroRuleDeclaration
    ($m:meta•+)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta             MacroParameterDeclaration
     $m                  McIdentifier
             +           PunctuationToken{tk: "+"}
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($m:meta ident) => {};                                                                                                                /*
    ($m:meta•ident)•=>•{}    MacroRuleDeclaration
    ($m:meta•ident)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                 MacroParameterDeclaration
     $m                      McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($m:meta $p:pat) => {};                                                                                                               /*
    ($m:meta•$p:pat)•=>•{}    MacroRuleDeclaration
    ($m:meta•$p:pat)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                  MacroParameterDeclaration
     $m                       McIdentifier
             $p:pat           MacroParameterDeclaration
             $p               McIdentifier
                        {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                    */
    ($m:meta $e:expr) => {};                                                                                                              /*
    ($m:meta•$e:expr)•=>•{}    MacroRuleDeclaration
    ($m:meta•$e:expr)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $e:expr           MacroParameterDeclaration
             $e                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($m:meta $t:ty) => {};                                                                                                                /*
    ($m:meta•$t:ty)•=>•{}    MacroRuleDeclaration
    ($m:meta•$t:ty)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                 MacroParameterDeclaration
     $m                      McIdentifier
             $t:ty           MacroParameterDeclaration
             $t              McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($m:meta $s:stmt) => {};                                                                                                              /*
    ($m:meta•$s:stmt)•=>•{}    MacroRuleDeclaration
    ($m:meta•$s:stmt)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $s:stmt           MacroParameterDeclaration
             $s                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($m:meta $p:path) => {};                                                                                                              /*
    ($m:meta•$p:path)•=>•{}    MacroRuleDeclaration
    ($m:meta•$p:path)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $p:path           MacroParameterDeclaration
             $p                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($m:meta $b:block) => {};                                                                                                             /*
    ($m:meta•$b:block)•=>•{}    MacroRuleDeclaration
    ($m:meta•$b:block)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $b:block           MacroParameterDeclaration
             $b                 McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($m:meta $i:ident) => {};                                                                                                             /*
    ($m:meta•$i:ident)•=>•{}    MacroRuleDeclaration
    ($m:meta•$i:ident)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $i:ident           MacroParameterDeclaration
             $i                 McIdentifier
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    ($m:meta $t:tt) => {};                                                                                                                /*
    ($m:meta•$t:tt)•=>•{}    MacroRuleDeclaration
    ($m:meta•$t:tt)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                 MacroParameterDeclaration
     $m                      McIdentifier
             $t:tt           MacroParameterDeclaration
             $t              McIdentifier
                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                     */
    ($m:meta $i:item) => {};                                                                                                              /*
    ($m:meta•$i:item)•=>•{}    MacroRuleDeclaration
    ($m:meta•$i:item)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $i:item           MacroParameterDeclaration
             $i                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($m:meta $n:meta) => {};                                                                                                              /*
    ($m:meta•$n:meta)•=>•{}    MacroRuleDeclaration
    ($m:meta•$n:meta)          MacroRuleDeclaration.match{dk: "()"}
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $n:meta           MacroParameterDeclaration
             $n                McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
	($ty:ty <) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
/*	($ty:ty•<)•=>•()                                                                          MacroRuleDeclaration
	($ty:ty•<)                                                                                MacroRuleDeclaration.match{dk: "()"}
	 $ty:ty                                                                                   MacroParameterDeclaration
	 $ty                                                                                      McIdentifier
	        <                                                                                 PunctuationToken{tk: "<"}
	              ()                                                                          MacroRuleDeclaration.transform{dk: "()"}    */
	                  //~•ERROR•`$ty:ty`•is•followed•by•`<`,•which•is•not•allowed•for•`ty`    Comment{line}
    ($ty:ty < foo ,) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
/*  ($ty:ty•<•foo•,)•=>•()                                                                          MacroRuleDeclaration
    ($ty:ty•<•foo•,)                                                                                MacroRuleDeclaration.match{dk: "()"}
     $ty:ty                                                                                         MacroParameterDeclaration
     $ty                                                                                            McIdentifier
            <                                                                                       PunctuationToken{tk: "<"}
                  ,                                                                                 PunctuationToken{tk: ","}
                        ()                                                                          MacroRuleDeclaration.transform{dk: "()"}*/
                            //~•ERROR•`$ty:ty`•is•followed•by•`<`,•which•is•not•allowed•for•`ty`    Comment{line}
    ($ty:ty , ) => ();                                                                                                                    /*
    ($ty:ty•,•)•=>•()    MacroRuleDeclaration
    ($ty:ty•,•)          MacroRuleDeclaration.match{dk: "()"}
     $ty:ty              MacroParameterDeclaration
     $ty                 McIdentifier
            ,            PunctuationToken{tk: ","}
                   ()    MacroRuleDeclaration.transform{dk: "()"}                                                                         */
    ( ( $ty:ty ) ) => ();                                                                                                                 /*
    (•(•$ty:ty•)•)•=>•()    MacroRuleDeclaration
    (•(•$ty:ty•)•)          MacroRuleDeclaration.match{dk: "()"}
      (•$ty:ty•)            DelimGroup
        $ty:ty              MacroParameterDeclaration
        $ty                 McIdentifier
                      ()    MacroRuleDeclaration.transform{dk: "()"}                                                                      */
    ( { $ty:ty } ) => ();                                                                                                                 /*
    (•{•$ty:ty•}•)•=>•()    MacroRuleDeclaration
    (•{•$ty:ty•}•)          MacroRuleDeclaration.match{dk: "()"}
      {•$ty:ty•}            DelimGroup
        $ty:ty              MacroParameterDeclaration
        $ty                 McIdentifier
                      ()    MacroRuleDeclaration.transform{dk: "()"}                                                                      */
    ( [ $ty:ty ] ) => ();                                                                                                                 /*
    (•[•$ty:ty•]•)•=>•()    MacroRuleDeclaration
    (•[•$ty:ty•]•)          MacroRuleDeclaration.match{dk: "()"}
      [•$ty:ty•]            DelimGroup
        $ty:ty              MacroParameterDeclaration
        $ty                 McIdentifier
                      ()    MacroRuleDeclaration.transform{dk: "()"}                                                                      */
    ($bl:block < ) => ();                                                                                                                 /*
    ($bl:block•<•)•=>•()    MacroRuleDeclaration
    ($bl:block•<•)          MacroRuleDeclaration.match{dk: "()"}
     $bl:block              MacroParameterDeclaration
     $bl                    McIdentifier
               <            PunctuationToken{tk: "<"}
                      ()    MacroRuleDeclaration.transform{dk: "()"}                                                                      */
    ($pa:pat >) => (); //~ ERROR `$pa:pat` is followed by `>`, which is not allowed for `pat`
/*  ($pa:pat•>)•=>•()                                                                            MacroRuleDeclaration
    ($pa:pat•>)                                                                                  MacroRuleDeclaration.match{dk: "()"}
     $pa:pat                                                                                     MacroParameterDeclaration
     $pa                                                                                         McIdentifier
             >                                                                                   PunctuationToken{tk: ">"}
                   ()                                                                            MacroRuleDeclaration.transform{dk: "()"} */
                       //~•ERROR•`$pa:pat`•is•followed•by•`>`,•which•is•not•allowed•for•`pat`    Comment{line}
    ($pa:pat , ) => ();                                                                                                                   /*
    ($pa:pat•,•)•=>•()    MacroRuleDeclaration
    ($pa:pat•,•)          MacroRuleDeclaration.match{dk: "()"}
     $pa:pat              MacroParameterDeclaration
     $pa                  McIdentifier
             ,            PunctuationToken{tk: ","}
                    ()    MacroRuleDeclaration.transform{dk: "()"}                                                                        */
    ($pa:pat $pb:pat $ty:ty ,) => ();                                                                                                     /*
    ($pa:pat•$pb:pat•$ty:ty•,)•=>•()    MacroRuleDeclaration
    ($pa:pat•$pb:pat•$ty:ty•,)          MacroRuleDeclaration.match{dk: "()"}
     $pa:pat                            MacroParameterDeclaration
     $pa                                McIdentifier
             $pb:pat                    MacroParameterDeclaration
             $pb                        McIdentifier
                     $ty:ty             MacroParameterDeclaration
                     $ty                McIdentifier
                            ,           PunctuationToken{tk: ","}
                                  ()    MacroRuleDeclaration.transform{dk: "()"}                                                          */
    //~^ ERROR `$pa:pat` is followed by `$pb:pat`, which is not allowed
    //~^•ERROR•`$pa:pat`•is•followed•by•`$pb:pat`,•which•is•not•allowed    Comment{line}
    //~^^ ERROR `$pb:pat` is followed by `$ty:ty`, which is not allowed
    //~^^•ERROR•`$pb:pat`•is•followed•by•`$ty:ty`,•which•is•not•allowed    Comment{line}
    ($($ty:ty)* -) => (); //~ ERROR `$ty:ty` is followed by `-`
/*  ($($ty:ty)*•-)•=>•()                                           MacroRuleDeclaration
    ($($ty:ty)*•-)                                                 MacroRuleDeclaration.match{dk: "()"}
     $($ty:ty)*                                                    MacroGroup{kind: "*"}
      ($ty:ty)                                                     MacroGroup.segments{dk: "()"}
       $ty:ty                                                      MacroParameterDeclaration
       $ty                                                         McIdentifier
                -                                                  PunctuationToken{tk: "-"}
                      ()                                           MacroRuleDeclaration.transform{dk: "()"}                               */
                          //~•ERROR•`$ty:ty`•is•followed•by•`-`    Comment{line}
    ($($a:ty, $b:ty)* -) => (); //~ ERROR `$b:ty` is followed by `-`
/*  ($($a:ty,•$b:ty)*•-)•=>•()                                          MacroRuleDeclaration
    ($($a:ty,•$b:ty)*•-)                                                MacroRuleDeclaration.match{dk: "()"}
     $($a:ty,•$b:ty)*                                                   MacroGroup{kind: "*"}
      ($a:ty,•$b:ty)                                                    MacroGroup.segments{dk: "()"}
       $a:ty                                                            MacroParameterDeclaration
       $a                                                               McIdentifier
            ,                                                           PunctuationToken{tk: ","}
              $b:ty                                                     MacroParameterDeclaration
              $b                                                        McIdentifier
                      -                                                 PunctuationToken{tk: "-"}
                            ()                                          MacroRuleDeclaration.transform{dk: "()"}                          */
                                //~•ERROR•`$b:ty`•is•followed•by•`-`    Comment{line}
    ($($ty:ty)-+) => (); //~ ERROR `$ty:ty` is followed by `-`, which is not allowed for `ty`
/*  ($($ty:ty)-+)•=>•()                                                                          MacroRuleDeclaration
    ($($ty:ty)-+)                                                                                MacroRuleDeclaration.match{dk: "()"}
     $($ty:ty)-+                                                                                 MacroGroup{kind: "+"}
      ($ty:ty)                                                                                   MacroGroup.segments{dk: "()"}
       $ty:ty                                                                                    MacroParameterDeclaration
       $ty                                                                                       McIdentifier
              -                                                                                  PunctuationToken{tk: "-"}
                     ()                                                                          MacroRuleDeclaration.transform{dk: "()"} */
                         //~•ERROR•`$ty:ty`•is•followed•by•`-`,•which•is•not•allowed•for•`ty`    Comment{line}
    ( $($a:expr)* $($b:tt)* ) => { };                                                                                                     /*
    (•$($a:expr)*•$($b:tt)*•)•=>•{•}    MacroRuleDeclaration
    (•$($a:expr)*•$($b:tt)*•)           MacroRuleDeclaration.match{dk: "()"}
      $($a:expr)*                       MacroGroup{kind: "*"}
       ($a:expr)                        MacroGroup.segments{dk: "()"}
        $a:expr                         MacroParameterDeclaration
        $a                              McIdentifier
                  $($b:tt)*             MacroGroup{kind: "*"}
                   ($b:tt)              MacroGroup.segments{dk: "()"}
                    $b:tt               MacroParameterDeclaration
                    $b                  McIdentifier
                                 {•}    MacroRuleDeclaration.transform{dk: "{}"}                                                          */
    //~^ ERROR `$a:expr` is followed by `$b:tt`, which is not allowed for `expr` fragments
    //~^•ERROR•`$a:expr`•is•followed•by•`$b:tt`,•which•is•not•allowed•for•`expr`•fragments    Comment{line}
    ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :                                                                 /*
    ($b•:•block,•$e•:•expr,•$i•:•ident,•$it•:•item,•$l•:•lifetime,•$lit•:↲    <MacroRuleDeclaration>
    ($b•:•block,•$e•:•expr,•$i•:•ident,•$it•:•item,•$l•:•lifetime,•$lit•:↲    <MacroRuleDeclaration.match{dk: "()"}>
     $b•:•block                                                               MacroParameterDeclaration
     $b                                                                       McIdentifier
               ,                                                              PunctuationToken{tk: ","}
                 $e•:•expr                                                    MacroParameterDeclaration
                 $e                                                           McIdentifier
                          ,                                                   PunctuationToken{tk: ","}
                            $i•:•ident                                        MacroParameterDeclaration
                            $i                                                McIdentifier
                                      ,                                       PunctuationToken{tk: ","}
                                        $it•:•item                            MacroParameterDeclaration
                                        $it                                   McIdentifier
                                                  ,                           PunctuationToken{tk: ","}
                                                    $l•:•lifetime             MacroParameterDeclaration
                                                    $l                        McIdentifier
                                                                 ,            PunctuationToken{tk: ","}
                                                                   $lit•:↲    <MacroParameterDeclaration>
                                                                   $lit       McIdentifier                                                */
     literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,                                                            /*
•••••literal                                                                      </MacroParameterDeclaration>
            ,                                                                     PunctuationToken{tk: ","}
              $m•:•meta                                                           MacroParameterDeclaration
              $m                                                                  McIdentifier
                       ,                                                          PunctuationToken{tk: ","}
                         $p•:•pat                                                 MacroParameterDeclaration
                         $p                                                       McIdentifier
                                 ,                                                PunctuationToken{tk: ","}
                                   $pth•:•path                                    MacroParameterDeclaration
                                   $pth                                           McIdentifier
                                              ,                                   PunctuationToken{tk: ","}
                                                $s•:•stmt                         MacroParameterDeclaration
                                                $s                                McIdentifier
                                                         ,                        PunctuationToken{tk: ","}
                                                           $tt•:•tt               MacroParameterDeclaration
                                                           $tt                    McIdentifier
                                                                   ,              PunctuationToken{tk: ","}
                                                                     $ty•:•ty     MacroParameterDeclaration
                                                                     $ty          McIdentifier
                                                                             ,    PunctuationToken{tk: ","}                               */
     $vis : vis) => { } ;                                                                                                                 /*
     $vis•:•vis            MacroParameterDeclaration
     $vis                  McIdentifier
•••••$vis•:•vis)           </MacroRuleDeclaration.match>
                    {•}    MacroRuleDeclaration.transform{dk: "{}"}
•••••$vis•:•vis)•=>•{•}    </MacroRuleDeclaration>                                                                                        */
}                                                                                                                                         /*
}    </MacroRulesDeclaration.rules>
}    </MacroRulesDeclaration>                                                                                                             */

#![rustc_dummy("hi", 1, 2, 1.012, pi = 3.14, bye, name("John"))]                                                                          /*
#![rustc_dummy("hi",•1,•2,•1.012,•pi•=•3.14,•bye,•name("John"))]    Attribute{inner}
  [rustc_dummy("hi",•1,•2,•1.012,•pi•=•3.14,•bye,•name("John"))]    Attribute.segments{dk: "[]"}
              ("hi",•1,•2,•1.012,•pi•=•3.14,•bye,•name("John"))     DelimGroup
               "hi"                                                 Literal{kind: String}
                   ,                                                PunctuationToken{tk: ","}
                     1                                              Literal{kind: Integer}
                      ,                                             PunctuationToken{tk: ","}
                        2                                           Literal{kind: Integer}
                         ,                                          PunctuationToken{tk: ","}
                           1.012                                    Literal{kind: Float}
                                ,                                   PunctuationToken{tk: ","}
                                     =                              PunctuationToken{tk: "="}
                                       3.14                         Literal{kind: Float}
                                           ,                        PunctuationToken{tk: ","}
                                                ,                   PunctuationToken{tk: ","}
                                                      ("John")      DelimGroup
                                                       "John"       Literal{kind: String}                                                 */
#[rustfmt::r#final(final)]                                                                                                                /*
#[rustfmt::r#final(final)]↲    <ExpressionStatement{!semi}>
#[rustfmt::r#final(final)]     Attribute{!inner}
 [rustfmt::r#final(final)]     Attribute.segments{dk: "[]"}
         ::                    PunctuationToken{tk: "::"}
                  (final)      DelimGroup                                                                                                 */

lexes!{a #foo}                                                                                                                            /*
lexes!{a•#foo}    ExpressionStatement~ownStart
lexes!{a•#foo}    MacroInvocation
      {a•#foo}    MacroInvocation.segments{dk: "{}"}
         #        PunctuationToken{tk: "#"}
lexes!{a•#foo}    </ExpressionStatement>                                                                                                  */
lexes!{continue 'foo}                                                                                                                     /*
lexes!{continue•'foo}    ExpressionStatement{!semi}, MacroInvocation
      {continue•'foo}    MacroInvocation.segments{dk: "{}"}
                'foo     LtIdentifier                                                                                                     */
lexes!{match "..." {}}                                                                                                                    /*
lexes!{match•"..."•{}}    ExpressionStatement{!semi}, MacroInvocation
      {match•"..."•{}}    MacroInvocation.segments{dk: "{}"}
             "..."        Literal{kind: String}
                   {}     DelimGroup                                                                                                      */
lexes!{r#let#foo} // Identifier<"r#let"; raw: true> PunctuationToken<#> Identifier<"foo">
                                                                                                                                          /*
lexes!{r#let#foo}                                                                            ExpressionStatement{!semi}, MacroInvocation
      {r#let#foo}                                                                            MacroInvocation.segments{dk: "{}"}
            #                                                                                PunctuationToken{tk: "#"}                    */
                  //•Identifier<"r#let";•raw:•true>•PunctuationToken<#>•Identifier<"foo">    Comment{line}

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>
    ()       FunctionDeclaration.parameters{dk: "()"}
       {↲    <FunctionDeclaration.body{dk: "{}"}>                                                                                         */
    unsafe {                                                                                                                              /*
    unsafe•{↲    <ExpressionStatement{!semi}>
    unsafe•{↲    <BlockExpression{unsafe}>
           {↲    <BlockExpression.body{dk: "{}"}>                                                                                         */
        asm!("");                                                                                                                         /*
        asm!("");    ExpressionStatement{semi}
        asm!("")     MacroInvocation
            ("")     MacroInvocation.segments{dk: "()"}
             ""      Literal{kind: String}                                                                                                */
        asm!("", options());                                                                                                              /*
        asm!("",•options());    ExpressionStatement{semi}
        asm!("",•options())     MacroInvocation
            ("",•options())     MacroInvocation.segments{dk: "()"}
             ""                 Literal{kind: String}
               ,                PunctuationToken{tk: ","}
                        ()      DelimGroup                                                                                                */
        asm!("", options(nostack, nomem));                                                                                                /*
        asm!("",•options(nostack,•nomem));    ExpressionStatement{semi}
        asm!("",•options(nostack,•nomem))     MacroInvocation
            ("",•options(nostack,•nomem))     MacroInvocation.segments{dk: "()"}
             ""                               Literal{kind: String}
               ,                              PunctuationToken{tk: ","}
                        (nostack,•nomem)      DelimGroup
                                ,             PunctuationToken{tk: ","}                                                                   */
        asm!("{}", in(reg) 4);                                                                                                            /*
        asm!("{}",•in(reg)•4);    ExpressionStatement{semi}
        asm!("{}",•in(reg)•4)     MacroInvocation
            ("{}",•in(reg)•4)     MacroInvocation.segments{dk: "()"}
             "{}"                 Literal{kind: String}
                 ,                PunctuationToken{tk: ","}
                     (reg)        DelimGroup
                           4      Literal{kind: Integer}                                                                                  */
        asm!("{0}", out(reg) a);                                                                                                          /*
        asm!("{0}",•out(reg)•a);    ExpressionStatement{semi}
        asm!("{0}",•out(reg)•a)     MacroInvocation
            ("{0}",•out(reg)•a)     MacroInvocation.segments{dk: "()"}
             "{0}"                  Literal{kind: String}
                  ,                 PunctuationToken{tk: ","}
                       (reg)        DelimGroup                                                                                            */
        asm!("{name}", name = inout(reg) b);                                                                                              /*
        asm!("{name}",•name•=•inout(reg)•b);    ExpressionStatement{semi}
        asm!("{name}",•name•=•inout(reg)•b)     MacroInvocation
            ("{name}",•name•=•inout(reg)•b)     MacroInvocation.segments{dk: "()"}
             "{name}"                           Literal{kind: String}
                     ,                          PunctuationToken{tk: ","}
                            =                   PunctuationToken{tk: "="}
                                   (reg)        DelimGroup                                                                                */
        asm!("{} {}", out(reg) _, inlateout(reg) b => _);                                                                                 /*
        asm!("{}•{}",•out(reg)•_,•inlateout(reg)•b•=>•_);    ExpressionStatement{semi}
        asm!("{}•{}",•out(reg)•_,•inlateout(reg)•b•=>•_)     MacroInvocation
            ("{}•{}",•out(reg)•_,•inlateout(reg)•b•=>•_)     MacroInvocation.segments{dk: "()"}
             "{}•{}"                                         Literal{kind: String}
                    ,                                        PunctuationToken{tk: ","}
                         (reg)                               DelimGroup
                               _                             PunctuationToken{tk: "_"}
                                ,                            PunctuationToken{tk: ","}
                                           (reg)             DelimGroup
                                                   =>        PunctuationToken{tk: "=>"}
                                                      _      PunctuationToken{tk: "_"}                                                    */
        asm!("", out("al") _, lateout("rcx") _);                                                                                          /*
        asm!("",•out("al")•_,•lateout("rcx")•_);    ExpressionStatement{semi}
        asm!("",•out("al")•_,•lateout("rcx")•_)     MacroInvocation
            ("",•out("al")•_,•lateout("rcx")•_)     MacroInvocation.segments{dk: "()"}
             ""                                     Literal{kind: String}
               ,                                    PunctuationToken{tk: ","}
                    ("al")                          DelimGroup
                     "al"                           Literal{kind: String}
                           _                        PunctuationToken{tk: "_"}
                            ,                       PunctuationToken{tk: ","}
                                     ("rcx")        DelimGroup
                                      "rcx"         Literal{kind: String}
                                             _      PunctuationToken{tk: "_"}                                                             */
        asm!("beep~", "boop!");                                                                                                           /*
        asm!("beep~",•"boop!");    ExpressionStatement{semi}
        asm!("beep~",•"boop!")     MacroInvocation
            ("beep~",•"boop!")     MacroInvocation.segments{dk: "()"}
             "beep~"               Literal{kind: String}
                    ,              PunctuationToken{tk: ","}
                      "boop!"      Literal{kind: String}                                                                                  */
        asm!("beep~ {}, 42", "boop! {}, 24", in(reg) a, out(reg) b);                                                                      /*
        asm!("beep~•{},•42",•"boop!•{},•24",•in(reg)•a,•out(reg)•b);    ExpressionStatement{semi}
        asm!("beep~•{},•42",•"boop!•{},•24",•in(reg)•a,•out(reg)•b)     MacroInvocation
            ("beep~•{},•42",•"boop!•{},•24",•in(reg)•a,•out(reg)•b)     MacroInvocation.segments{dk: "()"}
             "beep~•{},•42"                                             Literal{kind: String}
                           ,                                            PunctuationToken{tk: ","}
                             "boop!•{},•24"                             Literal{kind: String}
                                           ,                            PunctuationToken{tk: ","}
                                               (reg)                    DelimGroup
                                                      ,                 PunctuationToken{tk: ","}
                                                           (reg)        DelimGroup                                                        */
        asm!("boop! {1}, 24", "beep~ {0}, 42", in(reg) a, out(reg) b);                                                                    /*
        asm!("boop!•{1},•24",•"beep~•{0},•42",•in(reg)•a,•out(reg)•b);    ExpressionStatement{semi}
        asm!("boop!•{1},•24",•"beep~•{0},•42",•in(reg)•a,•out(reg)•b)     MacroInvocation
            ("boop!•{1},•24",•"beep~•{0},•42",•in(reg)•a,•out(reg)•b)     MacroInvocation.segments{dk: "()"}
             "boop!•{1},•24"                                              Literal{kind: String}
                            ,                                             PunctuationToken{tk: ","}
                              "beep~•{0},•42"                             Literal{kind: String}
                                             ,                            PunctuationToken{tk: ","}
                                                 (reg)                    DelimGroup
                                                        ,                 PunctuationToken{tk: ","}
                                                             (reg)        DelimGroup                                                      */
        asm!("beep~ {}, 42", "boop! {name}, 24", in(reg) a, name = out(reg) b);                                                           /*
        asm!("beep~•{},•42",•"boop!•{name},•24",•in(reg)•a,•name•=•out(reg)•b);    ExpressionStatement{semi}
        asm!("beep~•{},•42",•"boop!•{name},•24",•in(reg)•a,•name•=•out(reg)•b)     MacroInvocation
            ("beep~•{},•42",•"boop!•{name},•24",•in(reg)•a,•name•=•out(reg)•b)     MacroInvocation.segments{dk: "()"}
             "beep~•{},•42"                                                        Literal{kind: String}
                           ,                                                       PunctuationToken{tk: ","}
                             "boop!•{name},•24"                                    Literal{kind: String}
                                               ,                                   PunctuationToken{tk: ","}
                                                   (reg)                           DelimGroup
                                                          ,                        PunctuationToken{tk: ","}
                                                                 =                 PunctuationToken{tk: "="}
                                                                      (reg)        DelimGroup                                             */
        asm!(                                                                                                                             /*
        asm!(↲    <ExpressionStatement{semi}>
        asm!(↲    <MacroInvocation>
            (↲    <MacroInvocation.segments{dk: "()"}>                                                                                    */
            "beep~                                                                                                                       "/*
            "beep~↲    <Literal{kind: String}>                                                                                           */"
boop!"                                                                                                                                    /*
boop!"    </Literal>                                                                                                                      */
        );                                                                                                                                /*
••••••••)     </MacroInvocation.segments>
••••••••)     </MacroInvocation>
••••••••);    </ExpressionStatement>                                                                                                      */
        asm!("beep~\nboop!");                                                                                                             /*
        asm!("beep~\nboop!");    ExpressionStatement{semi}
        asm!("beep~\nboop!")     MacroInvocation
            ("beep~\nboop!")     MacroInvocation.segments{dk: "()"}
             "beep~\nboop!"      Literal{kind: String}                                                                                    */
        asm!("beep~\n\tboop!");                                                                                                           /*
        asm!("beep~\n\tboop!");    ExpressionStatement{semi}
        asm!("beep~\n\tboop!")     MacroInvocation
            ("beep~\n\tboop!")     MacroInvocation.segments{dk: "()"}
             "beep~\n\tboop!"      Literal{kind: String}                                                                                  */
        asm!("beep~\nboop!", "boop3\nboop4");                                                                                             /*
        asm!("beep~\nboop!",•"boop3\nboop4");    ExpressionStatement{semi}
        asm!("beep~\nboop!",•"boop3\nboop4")     MacroInvocation
            ("beep~\nboop!",•"boop3\nboop4")     MacroInvocation.segments{dk: "()"}
             "beep~\nboop!"                      Literal{kind: String}
                           ,                     PunctuationToken{tk: ","}
                             "boop3\nboop4"      Literal{kind: String}                                                                    */
    }                                                                                                                                     /*
••••}    </BlockExpression.body>
••••}    </BlockExpression>
••••}    </ExpressionStatement>                                                                                                           */
}                                                                                                                                         /*
}    </FunctionDeclaration.body>
}    </FunctionDeclaration>                                                                                                               */
debug!(?value);                                                                                                                           /*
debug!(?value);    ExpressionStatement{semi}
debug!(?value)     MacroInvocation
      (?value)     MacroInvocation.segments{dk: "()"}
       ?           PunctuationToken{tk: "?"}                                                                                              */
debug!(                                                                                                                                   /*
debug!(↲    <ExpressionStatement{semi}>
debug!(↲    <MacroInvocation>
      (↲    <MacroInvocation.segments{dk: "()"}>                                                                                          */
    "VariantDef::new(name = {:?}, variant_did = {:?}, ctor_def_id = {:?}, discr = {:?},                                                  "/*
    "VariantDef::new(name•=•{:?},•variant_did•=•{:?},•ctor_def_id•=•{:?},•discr•=•{:?},↲    <Literal{kind: String}>                      */"
     fields = {:?}, ctor_kind = {:?}, adt_kind = {:?}, parent_did = {:?})",                                                               /*
•••••fields•=•{:?},•ctor_kind•=•{:?},•adt_kind•=•{:?},•parent_did•=•{:?})"     </Literal>
                                                                          ,    PunctuationToken{tk: ","}                                  */
    name, variant_did, ctor_def_id, discr, fields, ctor_kind, adt_kind, parent_did,                                                       /*
        ,                                                                              PunctuationToken{tk: ","}
                     ,                                                                 PunctuationToken{tk: ","}
                                  ,                                                    PunctuationToken{tk: ","}
                                         ,                                             PunctuationToken{tk: ","}
                                                 ,                                     PunctuationToken{tk: ","}
                                                            ,                          PunctuationToken{tk: ","}
                                                                      ,                PunctuationToken{tk: ","}
                                                                                  ,    PunctuationToken{tk: ","}                          */
);                                                                                                                                        /*
)     </MacroInvocation.segments>
)     </MacroInvocation>
);    </ExpressionStatement>                                                                                                              */
slice_interners!(                                                                                                                         /*
slice_interners!(↲    <ExpressionStatement{semi}>
slice_interners!(↲    <MacroInvocation>
                (↲    <MacroInvocation.segments{dk: "()"}>                                                                                */
    substs: _intern_substs(GenericArg<'tcx>),                                                                                             /*
          :                                      PunctuationToken{tk: ":"}
                          (GenericArg<'tcx>)     DelimGroup
                                     <           PunctuationToken{tk: "<"}
                                      'tcx       LtIdentifier
                                          >      PunctuationToken{tk: ">"}
                                            ,    PunctuationToken{tk: ","}                                                                */
    canonical_var_infos: _intern_canonical_var_infos(CanonicalVarInfo<'tcx>),                                                             /*
                       :                                                         PunctuationToken{tk: ":"}
                                                    (CanonicalVarInfo<'tcx>)     DelimGroup
                                                                     <           PunctuationToken{tk: "<"}
                                                                      'tcx       LtIdentifier
                                                                          >      PunctuationToken{tk: ">"}
                                                                            ,    PunctuationToken{tk: ","}                                */
    poly_existential_predicates:                                                                                                          /*
                               :    PunctuationToken{tk: ":"}                                                                             */
        _intern_poly_existential_predicates(ty::Binder<'tcx, ExistentialPredicate<'tcx>>),                                                /*
                                           (ty::Binder<'tcx,•ExistentialPredicate<'tcx>>)     DelimGroup
                                              ::                                              PunctuationToken{tk: "::"}
                                                      <                                       PunctuationToken{tk: "<"}
                                                       'tcx                                   LtIdentifier
                                                           ,                                  PunctuationToken{tk: ","}
                                                                                 <            PunctuationToken{tk: "<"}
                                                                                  'tcx        LtIdentifier
                                                                                      >>      PunctuationToken{tk: ">>"}
                                                                                         ,    PunctuationToken{tk: ","}                   */
    predicates: _intern_predicates(Predicate<'tcx>),                                                                                      /*
              :                                         PunctuationToken{tk: ":"}
                                  (Predicate<'tcx>)     DelimGroup
                                            <           PunctuationToken{tk: "<"}
                                             'tcx       LtIdentifier
                                                 >      PunctuationToken{tk: ">"}
                                                   ,    PunctuationToken{tk: ","}                                                         */
    projs: _intern_projs(ProjectionKind),                                                                                                 /*
         :                                   PunctuationToken{tk: ":"}
                        (ProjectionKind)     DelimGroup
                                        ,    PunctuationToken{tk: ","}                                                                    */
    place_elems: _intern_place_elems(PlaceElem<'tcx>),                                                                                    /*
               :                                          PunctuationToken{tk: ":"}
                                    (PlaceElem<'tcx>)     DelimGroup
                                              <           PunctuationToken{tk: "<"}
                                               'tcx       LtIdentifier
                                                   >      PunctuationToken{tk: ">"}
                                                     ,    PunctuationToken{tk: ","}                                                       */
    bound_variable_kinds: _intern_bound_variable_kinds(ty::BoundVariableKind),                                                            /*
                        :                                                         PunctuationToken{tk: ":"}
                                                      (ty::BoundVariableKind)     DelimGroup
                                                         ::                       PunctuationToken{tk: "::"}
                                                                             ,    PunctuationToken{tk: ","}                               */
);                                                                                                                                        /*
)     </MacroInvocation.segments>
)     </MacroInvocation>
);    </ExpressionStatement>                                                                                                              */

impl_binder_encode_decode! {                                                                                                              /*
impl_binder_encode_decode!•{↲    <ExpressionStatement{!semi}>
impl_binder_encode_decode!•{↲    <MacroInvocation>
                           {↲    <MacroInvocation.segments{dk: "{}"}>                                                                     */
    &'tcx ty::List<Ty<'tcx>>,                                                                                                             /*
    &                            PunctuationToken{tk: "&"}
     'tcx                        LtIdentifier
            ::                   PunctuationToken{tk: "::"}
                  <              PunctuationToken{tk: "<"}
                     <           PunctuationToken{tk: "<"}
                      'tcx       LtIdentifier
                          >>     PunctuationToken{tk: ">>"}
                            ,    PunctuationToken{tk: ","}                                                                                */
    ty::FnSig<'tcx>,                                                                                                                      /*
      ::                PunctuationToken{tk: "::"}
             <          PunctuationToken{tk: "<"}
              'tcx      LtIdentifier
                  >     PunctuationToken{tk: ">"}
                   ,    PunctuationToken{tk: ","}                                                                                         */
    ty::ExistentialPredicate<'tcx>,                                                                                                       /*
      ::                               PunctuationToken{tk: "::"}
                            <          PunctuationToken{tk: "<"}
                             'tcx      LtIdentifier
                                 >     PunctuationToken{tk: ">"}
                                  ,    PunctuationToken{tk: ","}                                                                          */
    ty::TraitRef<'tcx>,                                                                                                                   /*
      ::                   PunctuationToken{tk: "::"}
                <          PunctuationToken{tk: "<"}
                 'tcx      LtIdentifier
                     >     PunctuationToken{tk: ">"}
                      ,    PunctuationToken{tk: ","}                                                                                      */
    Vec<ty::GeneratorInteriorTypeCause<'tcx>>,                                                                                            /*
       <                                          PunctuationToken{tk: "<"}
          ::                                      PunctuationToken{tk: "::"}
                                      <           PunctuationToken{tk: "<"}
                                       'tcx       LtIdentifier
                                           >>     PunctuationToken{tk: ">>"}
                                             ,    PunctuationToken{tk: ","}                                                               */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */
impl_arena_copy_decoder! {<'tcx>                                                                                                          /*
impl_arena_copy_decoder!•{<'tcx>↲    <ExpressionStatement{!semi}>
impl_arena_copy_decoder!•{<'tcx>↲    <MacroInvocation>
                         {<'tcx>↲    <MacroInvocation.segments{dk: "{}"}>
                          <          PunctuationToken{tk: "<"}
                           'tcx      LtIdentifier
                               >     PunctuationToken{tk: ">"}                                                                            */
    Span,                                                                                                                                 /*
        ,    PunctuationToken{tk: ","}                                                                                                    */
    rustc_span::symbol::Ident,                                                                                                            /*
              ::                  PunctuationToken{tk: "::"}
                      ::          PunctuationToken{tk: "::"}
                             ,    PunctuationToken{tk: ","}                                                                               */
    ty::Variance,                                                                                                                         /*
      ::             PunctuationToken{tk: "::"}
                ,    PunctuationToken{tk: ","}                                                                                            */
    rustc_span::def_id::DefId,                                                                                                            /*
              ::                  PunctuationToken{tk: "::"}
                      ::          PunctuationToken{tk: "::"}
                             ,    PunctuationToken{tk: ","}                                                                               */
    rustc_span::def_id::LocalDefId,                                                                                                       /*
              ::                       PunctuationToken{tk: "::"}
                      ::               PunctuationToken{tk: "::"}
                                  ,    PunctuationToken{tk: ","}                                                                          */
    (rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>, rustc_middle::middle::exported_symbols::SymbolExportInfo),             /*
    (rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>,•rustc_middle::middle::exported_symbols::SymbolExportInfo)     DelimGroup
                 ::                                                                                                              PunctuationToken{tk: "::"}
                         ::                                                                                                      PunctuationToken{tk: "::"}
                                           ::                                                                                    PunctuationToken{tk: "::"}
                                                           <                                                                     PunctuationToken{tk: "<"}
                                                            'tcx                                                                 LtIdentifier
                                                                >                                                                PunctuationToken{tk: ">"}
                                                                 ,                                                               PunctuationToken{tk: ","}
                                                                               ::                                                PunctuationToken{tk: "::"}
                                                                                       ::                                        PunctuationToken{tk: "::"}
                                                                                                         ::                      PunctuationToken{tk: "::"}
                                                                                                                            ,    PunctuationToken{tk: ","}*/
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */

bitflags! {                                                                                                                               /*
bitflags!•{↲    <ExpressionStatement{!semi}>
bitflags!•{↲    <MacroInvocation>
          {↲    <MacroInvocation.segments{dk: "{}"}>                                                                                      */
    #[derive(HashStable, TyEncodable, TyDecodable)]                                                                                       /*
    #                                                  PunctuationToken{tk: "#"}
     [derive(HashStable,•TyEncodable,•TyDecodable)]    DelimGroup
            (HashStable,•TyEncodable,•TyDecodable)     DelimGroup
                       ,                               PunctuationToken{tk: ","}
                                    ,                  PunctuationToken{tk: ","}                                                          */
    pub struct AdtFlags: u32 {                                                                                                            /*
                       :           PunctuationToken{tk: ":"}
                             {↲    <DelimGroup>                                                                                           */
        const NO_ADT_FLAGS        = 0;                                                                                                    /*
                                  =       PunctuationToken{tk: "="}
                                    0     Literal{kind: Integer}
                                     ;    PunctuationToken{tk: ";"}                                                                       */
        /// Indicates whether the ADT is an enum.
        ///•Indicates•whether•the•ADT•is•an•enum.    Comment{line}
        const IS_ENUM             = 1 << 0;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         0     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the ADT is a union.
        ///•Indicates•whether•the•ADT•is•a•union.    Comment{line}
        const IS_UNION            = 1 << 1;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         1     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the ADT is a struct.
        ///•Indicates•whether•the•ADT•is•a•struct.    Comment{line}
        const IS_STRUCT           = 1 << 2;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         2     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the ADT is a struct and has a constructor.
        ///•Indicates•whether•the•ADT•is•a•struct•and•has•a•constructor.    Comment{line}
        const HAS_CTOR            = 1 << 3;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         3     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the type is `PhantomData`.
        ///•Indicates•whether•the•type•is•`PhantomData`.    Comment{line}
        const IS_PHANTOM_DATA     = 1 << 4;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         4     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the type has a `#[fundamental]` attribute.
        ///•Indicates•whether•the•type•has•a•`#[fundamental]`•attribute.    Comment{line}
        const IS_FUNDAMENTAL      = 1 << 5;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         5     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the type is `Box`.
        ///•Indicates•whether•the•type•is•`Box`.    Comment{line}
        const IS_BOX              = 1 << 6;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         6     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the type is `ManuallyDrop`.
        ///•Indicates•whether•the•type•is•`ManuallyDrop`.    Comment{line}
        const IS_MANUALLY_DROP    = 1 << 7;                                                                                               /*
                                  =            PunctuationToken{tk: "="}
                                    1          Literal{kind: Integer}
                                      <<       PunctuationToken{tk: "<<"}
                                         7     Literal{kind: Integer}
                                          ;    PunctuationToken{tk: ";"}                                                                  */
        /// Indicates whether the variant list of this ADT is `#[non_exhaustive]`.
        ///•Indicates•whether•the•variant•list•of•this•ADT•is•`#[non_exhaustive]`.    Comment{line}
        /// (i.e., this flag is never set unless this ADT is an enum).
        ///•(i.e.,•this•flag•is•never•set•unless•this•ADT•is•an•enum).    Comment{line}
        const IS_VARIANT_LIST_NON_EXHAUSTIVE = 1 << 8;                                                                                    /*
                                             =            PunctuationToken{tk: "="}
                                               1          Literal{kind: Integer}
                                                 <<       PunctuationToken{tk: "<<"}
                                                    8     Literal{kind: Integer}
                                                     ;    PunctuationToken{tk: ";"}                                                       */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </MacroInvocation.segments>
}    </MacroInvocation>
}    </ExpressionStatement>                                                                                                               */
rustc_dep_node_append!([define_dep_nodes!][ <'tcx>                                                                                        /*
rustc_dep_node_append!([define_dep_nodes!][•<'tcx>↲    <ExpressionStatement{semi}>
rustc_dep_node_append!([define_dep_nodes!][•<'tcx>↲    <MacroInvocation>
                      ([define_dep_nodes!][•<'tcx>↲    <MacroInvocation.segments{dk: "()"}>
                       [define_dep_nodes!]             DelimGroup
                                        !              PunctuationToken{tk: "!"}
                                          [•<'tcx>↲    <DelimGroup>
                                            <          PunctuationToken{tk: "<"}
                                             'tcx      LtIdentifier
                                                 >     PunctuationToken{tk: ">"}                                                          */
    // We use this for most things when incr. comp. is turned off.
    //•We•use•this•for•most•things•when•incr.•comp.•is•turned•off.    Comment{line}
    [] Null,                                                                                                                              /*
    []          DelimGroup
           ,    PunctuationToken{tk: ","}                                                                                                 */

    [anon] TraitSelect,                                                                                                                   /*
    [anon]                 DelimGroup
                      ,    PunctuationToken{tk: ","}                                                                                      */

    // WARNING: if `Symbol` is changed, make sure you update `make_compile_codegen_unit` below.
    //•WARNING:•if•`Symbol`•is•changed,•make•sure•you•update•`make_compile_codegen_unit`•below.    Comment{line}
    [] CompileCodegenUnit(Symbol),                                                                                                        /*
    []                                DelimGroup
                         (Symbol)     DelimGroup
                                 ,    PunctuationToken{tk: ","}                                                                           */

    // WARNING: if `MonoItem` is changed, make sure you update `make_compile_mono_item` below.
    //•WARNING:•if•`MonoItem`•is•changed,•make•sure•you•update•`make_compile_mono_item`•below.    Comment{line}
    // Only used by rustc_codegen_cranelift
    //•Only•used•by•rustc_codegen_cranelift    Comment{line}
    [] CompileMonoItem(MonoItem),                                                                                                         /*
    []                               DelimGroup
                      (MonoItem)     DelimGroup
                                ,    PunctuationToken{tk: ","}                                                                            */
]);                                                                                                                                       /*
]      </DelimGroup>
])     </MacroInvocation.segments>
])     </MacroInvocation>
]);    </ExpressionStatement>                                                                                                             */

decl_derive!([Decodable] => serialize::decodable_derive);                                                                                 /*
decl_derive!([Decodable]•=>•serialize::decodable_derive);    ExpressionStatement{semi}
decl_derive!([Decodable]•=>•serialize::decodable_derive)     MacroInvocation
            ([Decodable]•=>•serialize::decodable_derive)     MacroInvocation.segments{dk: "()"}
             [Decodable]                                     DelimGroup
                         =>                                  PunctuationToken{tk: "=>"}
                                     ::                      PunctuationToken{tk: "::"}                                                   */

let ret = structure.gen_impl(quote! {                                                                                                     /*
let•ret•=•structure.gen_impl(quote!•{↲    <LetVariableDeclaration>
          structure.gen_impl(quote!•{↲    <CallExpression>
                            (quote!•{↲    <CallExpression.arguments{dk: "()"}>
                             quote!•{↲    <MacroInvocation>
                                    {↲    <MacroInvocation.segments{dk: "{}"}>                                                            */
    gen impl rustc_errors::AddSubdiagnostic for @Self {                                                                                   /*
                         ::                                 PunctuationToken{tk: "::"}
                                                @           PunctuationToken{tk: "@"}
                                                      {↲    <DelimGroup>                                                                  */
        fn add_to_diagnostic(self, #diag: &mut rustc_errors::Diagnostic) {                                                                /*
                            (self,•#diag:•&mut•rustc_errors::Diagnostic)       DelimGroup
                                 ,                                             PunctuationToken{tk: ","}
                                   #                                           PunctuationToken{tk: "#"}
                                        :                                      PunctuationToken{tk: ":"}
                                          &                                    PunctuationToken{tk: "&"}
                                                           ::                  PunctuationToken{tk: "::"}
                                                                         {↲    <DelimGroup>                                               */
            use rustc_errors::{Applicability, IntoDiagnosticArg};                                                                         /*
                            ::                                       PunctuationToken{tk: "::"}
                              {Applicability,•IntoDiagnosticArg}     DelimGroup
                                            ,                        PunctuationToken{tk: ","}
                                                                ;    PunctuationToken{tk: ";"}                                            */
            #implementation                                                                                                               /*
            #    PunctuationToken{tk: "#"}                                                                                                */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
});                                                                                                                                       /*
}      </MacroInvocation.segments>
}      </MacroInvocation>
})     </CallExpression.arguments>
})     </CallExpression>
});    </LetVariableDeclaration>
});    </Program.ast>
});    </Program>                                                                                                                         */
// Discarded Nodes: 0
// Parsed Nodes: 2242
// state_rollbacks: 2
// Total '.charCodeAt()' calls: 12504 (13% re-reads)
// Unnecessary 'skip_whitespace()' calls: 738
// source: "../../samples/macro/macro.tokens.rs"