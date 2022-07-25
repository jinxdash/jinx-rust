macro_rules! a {                                                                                                                          /*
macro_rules!•a•{↲    <MacroRulesDeclaration>                                                                                              */
	( + ) => { + }; ( += ) => { += }; ( & ) => { & }; ( && ) => { && };                                                                   /*
    (•+•)•=>•{•+•}                                                        MacroRuleDeclaration
      +                                                                   PunctuationToken
               +                                                          PunctuationToken
                    (•+=•)•=>•{•+=•}                                      MacroRuleDeclaration
                      +=                                                  PunctuationToken
                                +=                                        PunctuationToken
                                      (•&•)•=>•{•&•}                      MacroRuleDeclaration
                                        &                                 PunctuationToken
                                                 &                        PunctuationToken
                                                      (•&&•)•=>•{•&&•}    MacroRuleDeclaration
                                                        &&                PunctuationToken
                                                                  &&      PunctuationToken                                                */
	( &= ) => { &= }; ( @ ) => { @ }; ( ! ) => { ! }; ( ^ ) => { ^ };                                                                     /*
    (•&=•)•=>•{•&=•}                                                    MacroRuleDeclaration
      &=                                                                PunctuationToken
                &=                                                      PunctuationToken
                      (•@•)•=>•{•@•}                                    MacroRuleDeclaration
                        @                                               PunctuationToken
                                 @                                      PunctuationToken
                                      (•!•)•=>•{•!•}                    MacroRuleDeclaration
                                        !                               PunctuationToken
                                                 !                      PunctuationToken
                                                      (•^•)•=>•{•^•}    MacroRuleDeclaration
                                                        ^               PunctuationToken
                                                                 ^      PunctuationToken                                                  */
	( ^= ) => { ^= }; ( : ) => { : }; ( :: ) => { :: }; ( , ) => { , };                                                                   /*
    (•^=•)•=>•{•^=•}                                                      MacroRuleDeclaration
      ^=                                                                  PunctuationToken
                ^=                                                        PunctuationToken
                      (•:•)•=>•{•:•}                                      MacroRuleDeclaration
                        :                                                 PunctuationToken
                                 :                                        PunctuationToken
                                      (•::•)•=>•{•::•}                    MacroRuleDeclaration
                                        ::                                PunctuationToken
                                                  ::                      PunctuationToken
                                                        (•,•)•=>•{•,•}    MacroRuleDeclaration
                                                          ,               PunctuationToken
                                                                   ,      PunctuationToken                                                */
	( / ) => { / }; ( /= ) => { /= }; ( . ) => { . }; ( .. ) => { .. };                                                                   /*
    (•/•)•=>•{•/•}                                                        MacroRuleDeclaration
      /                                                                   PunctuationToken
               /                                                          PunctuationToken
                    (•/=•)•=>•{•/=•}                                      MacroRuleDeclaration
                      /=                                                  PunctuationToken
                                /=                                        PunctuationToken
                                      (•.•)•=>•{•.•}                      MacroRuleDeclaration
                                        .                                 PunctuationToken
                                                 .                        PunctuationToken
                                                      (•..•)•=>•{•..•}    MacroRuleDeclaration
                                                        ..                PunctuationToken
                                                                  ..      PunctuationToken                                                */
	( ... ) => { ... }; ( ..= ) => { ..= }; ( = ) => { = }; ( == ) => { == };                                                             /*
    (•...•)•=>•{•...•}                                                          MacroRuleDeclaration
      ...                                                                       PunctuationToken
                 ...                                                            PunctuationToken
                        (•..=•)•=>•{•..=•}                                      MacroRuleDeclaration
                          ..=                                                   PunctuationToken
                                     ..=                                        PunctuationToken
                                            (•=•)•=>•{•=•}                      MacroRuleDeclaration
                                              =                                 PunctuationToken
                                                       =                        PunctuationToken
                                                            (•==•)•=>•{•==•}    MacroRuleDeclaration
                                                              ==                PunctuationToken
                                                                        ==      PunctuationToken                                          */
	( >= ) => { >= }; ( > ) => { > }; ( <= ) => { <= }; ( < ) => { < };                                                                   /*
    (•>=•)•=>•{•>=•}                                                      MacroRuleDeclaration
      >=                                                                  PunctuationToken
                >=                                                        PunctuationToken
                      (•>•)•=>•{•>•}                                      MacroRuleDeclaration
                        >                                                 PunctuationToken
                                 >                                        PunctuationToken
                                      (•<=•)•=>•{•<=•}                    MacroRuleDeclaration
                                        <=                                PunctuationToken
                                                  <=                      PunctuationToken
                                                        (•<•)•=>•{•<•}    MacroRuleDeclaration
                                                          <               PunctuationToken
                                                                   <      PunctuationToken                                                */
	( *= ) => { *= }; ( != ) => { != }; ( | ) => { | }; ( |= ) => { |= };                                                                 /*
    (•*=•)•=>•{•*=•}                                                        MacroRuleDeclaration
      *=                                                                    PunctuationToken
                *=                                                          PunctuationToken
                      (•!=•)•=>•{•!=•}                                      MacroRuleDeclaration
                        !=                                                  PunctuationToken
                                  !=                                        PunctuationToken
                                        (•|•)•=>•{•|•}                      MacroRuleDeclaration
                                          |                                 PunctuationToken
                                                   |                        PunctuationToken
                                                        (•|=•)•=>•{•|=•}    MacroRuleDeclaration
                                                          |=                PunctuationToken
                                                                    |=      PunctuationToken                                              */
	( || ) => { || }; ( # ) => { # }; ( ? ) => { ? }; ( -> ) => { -> };                                                                   /*
    (•||•)•=>•{•||•}                                                      MacroRuleDeclaration
      ||                                                                  PunctuationToken
                ||                                                        PunctuationToken
                      (•#•)•=>•{•#•}                                      MacroRuleDeclaration
                        #                                                 PunctuationToken
                                 #                                        PunctuationToken
                                      (•?•)•=>•{•?•}                      MacroRuleDeclaration
                                        ?                                 PunctuationToken
                                                 ?                        PunctuationToken
                                                      (•->•)•=>•{•->•}    MacroRuleDeclaration
                                                        ->                PunctuationToken
                                                                  ->      PunctuationToken                                                */
	( <- ) => { <- }; ( % ) => { % }; ( %= ) => { %= }; ( => ) => { => };                                                                 /*
    (•<-•)•=>•{•<-•}                                                        MacroRuleDeclaration
      <                                                                     PunctuationToken
       -                                                                    PunctuationToken
                <                                                           PunctuationToken
                 -                                                          PunctuationToken
                      (•%•)•=>•{•%•}                                        MacroRuleDeclaration
                        %                                                   PunctuationToken
                                 %                                          PunctuationToken
                                      (•%=•)•=>•{•%=•}                      MacroRuleDeclaration
                                        %=                                  PunctuationToken
                                                  %=                        PunctuationToken
                                                        (•=>•)•=>•{•=>•}    MacroRuleDeclaration
                                                          =>                PunctuationToken
                                                                    =>      PunctuationToken                                              */
	( ; ) => { ; }; ( << ) => { << }; ( <<= ) => { <<= }; ( >> ) => { >> };                                                               /*
    (•;•)•=>•{•;•}                                                            MacroRuleDeclaration
      ;                                                                       PunctuationToken
               ;                                                              PunctuationToken
                    (•<<•)•=>•{•<<•}                                          MacroRuleDeclaration
                      <<                                                      PunctuationToken
                                <<                                            PunctuationToken
                                      (•<<=•)•=>•{•<<=•}                      MacroRuleDeclaration
                                        <<=                                   PunctuationToken
                                                   <<=                        PunctuationToken
                                                          (•>>•)•=>•{•>>•}    MacroRuleDeclaration
                                                            >>                PunctuationToken
                                                                      >>      PunctuationToken                                            */
	( >>= ) => { >>= }; ( * ) => { * }; ( - ) => { - }; ( -= ) => { -= }; ( ~ ) => { ~ };                                                 /*
    (•>>=•)•=>•{•>>=•}                                                                      MacroRuleDeclaration
      >>=                                                                                   PunctuationToken
                 >>=                                                                        PunctuationToken
                        (•*•)•=>•{•*•}                                                      MacroRuleDeclaration
                          *                                                                 PunctuationToken
                                   *                                                        PunctuationToken
                                        (•-•)•=>•{•-•}                                      MacroRuleDeclaration
                                          -                                                 PunctuationToken
                                                   -                                        PunctuationToken
                                                        (•-=•)•=>•{•-=•}                    MacroRuleDeclaration
                                                          -=                                PunctuationToken
                                                                    -=                      PunctuationToken
                                                                          (•~•)•=>•{•~•}    MacroRuleDeclaration
                                                                            ~               PunctuationToken
                                                                                     ~      PunctuationToken                              */
}                                                                                                                                         /*
}    </MacroRulesDeclaration>                                                                                                             */


a!{ + } a!{ += } a!{ & } a!{ && } a!{ &= } a!{ @ } a!{ ! } a!{ ^ } a!{ ^= }                                                               /*
a!{•+•}                                                                        ExpressionStatement, MacroInvocation
    +                                                                          PunctuationToken
        a!{•+=•}                                                               ExpressionStatement, MacroInvocation
            +=                                                                 PunctuationToken
                 a!{•&•}                                                       ExpressionStatement, MacroInvocation
                     &                                                         PunctuationToken
                         a!{•&&•}                                              ExpressionStatement, MacroInvocation
                             &&                                                PunctuationToken
                                  a!{•&=•}                                     ExpressionStatement, MacroInvocation
                                      &=                                       PunctuationToken
                                           a!{•@•}                             ExpressionStatement, MacroInvocation
                                               @                               PunctuationToken
                                                   a!{•!•}                     ExpressionStatement, MacroInvocation
                                                       !                       PunctuationToken
                                                           a!{•^•}             ExpressionStatement, MacroInvocation
                                                               ^               PunctuationToken
                                                                   a!{•^=•}    ExpressionStatement, MacroInvocation
                                                                       ^=      PunctuationToken                                           */
a!{ : } a!{ :: } a!{ , } a!{ / } a!{ /= } a!{ . } a!{ .. } a!{ ... } a!{ ..= }                                                            /*
a!{•:•}                                                                           ExpressionStatement, MacroInvocation
    :                                                                             PunctuationToken
        a!{•::•}                                                                  ExpressionStatement, MacroInvocation
            ::                                                                    PunctuationToken
                 a!{•,•}                                                          ExpressionStatement, MacroInvocation
                     ,                                                            PunctuationToken
                         a!{•/•}                                                  ExpressionStatement, MacroInvocation
                             /                                                    PunctuationToken
                                 a!{•/=•}                                         ExpressionStatement, MacroInvocation
                                     /=                                           PunctuationToken
                                          a!{•.•}                                 ExpressionStatement, MacroInvocation
                                              .                                   PunctuationToken
                                                  a!{•..•}                        ExpressionStatement, MacroInvocation
                                                      ..                          PunctuationToken
                                                           a!{•...•}              ExpressionStatement, MacroInvocation
                                                               ...                PunctuationToken
                                                                     a!{•..=•}    ExpressionStatement, MacroInvocation
                                                                         ..=      PunctuationToken                                        */
a!{ = } a!{ == } a!{ >= } a!{ > } a!{ <= } a!{ < } a!{ *= } a!{ != } a!{ | }                                                              /*
a!{•=•}                                                                         ExpressionStatement, MacroInvocation
    =                                                                           PunctuationToken
        a!{•==•}                                                                ExpressionStatement, MacroInvocation
            ==                                                                  PunctuationToken
                 a!{•>=•}                                                       ExpressionStatement, MacroInvocation
                     >=                                                         PunctuationToken
                          a!{•>•}                                               ExpressionStatement, MacroInvocation
                              >                                                 PunctuationToken
                                  a!{•<=•}                                      ExpressionStatement, MacroInvocation
                                      <=                                        PunctuationToken
                                           a!{•<•}                              ExpressionStatement, MacroInvocation
                                               <                                PunctuationToken
                                                   a!{•*=•}                     ExpressionStatement, MacroInvocation
                                                       *=                       PunctuationToken
                                                            a!{•!=•}            ExpressionStatement, MacroInvocation
                                                                !=              PunctuationToken
                                                                     a!{•|•}    ExpressionStatement, MacroInvocation
                                                                         |      PunctuationToken                                          */
a!{ |= } a!{ || } a!{ # } a!{ ? } a!{ -> } a!{ <- } a!{ % } a!{ %= } a!{ => }                                                             /*
a!{•|=•}                                                                         ExpressionStatement, MacroInvocation
    |=                                                                           PunctuationToken
         a!{•||•}                                                                ExpressionStatement, MacroInvocation
             ||                                                                  PunctuationToken
                  a!{•#•}                                                        ExpressionStatement, MacroInvocation
                      #                                                          PunctuationToken
                          a!{•?•}                                                ExpressionStatement, MacroInvocation
                              ?                                                  PunctuationToken
                                  a!{•->•}                                       ExpressionStatement, MacroInvocation
                                      ->                                         PunctuationToken
                                           a!{•<-•}                              ExpressionStatement, MacroInvocation
                                               <                                 PunctuationToken
                                                -                                PunctuationToken
                                                    a!{•%•}                      ExpressionStatement, MacroInvocation
                                                        %                        PunctuationToken
                                                            a!{•%=•}             ExpressionStatement, MacroInvocation
                                                                %=               PunctuationToken
                                                                     a!{•=>•}    ExpressionStatement, MacroInvocation
                                                                         =>      PunctuationToken                                         */
a!{ ; } a!{ << } a!{ <<= } a!{ >> } a!{ >>= } a!{ * } a!{ - } a!{ -= } a!{ ~ }                                                            /*
a!{•;•}                                                                           ExpressionStatement, MacroInvocation
    ;                                                                             PunctuationToken
        a!{•<<•}                                                                  ExpressionStatement, MacroInvocation
            <<                                                                    PunctuationToken
                 a!{•<<=•}                                                        ExpressionStatement, MacroInvocation
                     <<=                                                          PunctuationToken
                           a!{•>>•}                                               ExpressionStatement, MacroInvocation
                               >>                                                 PunctuationToken
                                    a!{•>>=•}                                     ExpressionStatement, MacroInvocation
                                        >>=                                       PunctuationToken
                                              a!{•*•}                             ExpressionStatement, MacroInvocation
                                                  *                               PunctuationToken
                                                      a!{•-•}                     ExpressionStatement, MacroInvocation
                                                          -                       PunctuationToken
                                                              a!{•-=•}            ExpressionStatement, MacroInvocation
                                                                  -=              PunctuationToken
                                                                       a!{•~•}    ExpressionStatement, MacroInvocation
                                                                           ~      PunctuationToken                                        */


b![ + ] b![ += ] b![ & ] b![ && ] b![ &= ] b![ @ ] b![ ! ] b![ ^ ] b![ ^= ]                                                               /*
b![•+•]                                                                        ExpressionStatement, MacroInvocation
    +                                                                          PunctuationToken
        b![•+=•]                                                               ExpressionStatement, MacroInvocation
            +=                                                                 PunctuationToken
                 b![•&•]                                                       ExpressionStatement, MacroInvocation
                     &                                                         PunctuationToken
                         b![•&&•]                                              ExpressionStatement, MacroInvocation
                             &&                                                PunctuationToken
                                  b![•&=•]                                     ExpressionStatement, MacroInvocation
                                      &=                                       PunctuationToken
                                           b![•@•]                             ExpressionStatement, MacroInvocation
                                               @                               PunctuationToken
                                                   b![•!•]                     ExpressionStatement, MacroInvocation
                                                       !                       PunctuationToken
                                                           b![•^•]             ExpressionStatement, MacroInvocation
                                                               ^               PunctuationToken
                                                                   b![•^=•]    ExpressionStatement, MacroInvocation
                                                                       ^=      PunctuationToken                                           */
b![ : ] b![ :: ] b![ , ] b![ / ] b![ /= ] b![ . ] b![ .. ] b![ ... ] b![ ..= ]                                                            /*
b![•:•]                                                                           ExpressionStatement, MacroInvocation
    :                                                                             PunctuationToken
        b![•::•]                                                                  ExpressionStatement, MacroInvocation
            ::                                                                    PunctuationToken
                 b![•,•]                                                          ExpressionStatement, MacroInvocation
                     ,                                                            PunctuationToken
                         b![•/•]                                                  ExpressionStatement, MacroInvocation
                             /                                                    PunctuationToken
                                 b![•/=•]                                         ExpressionStatement, MacroInvocation
                                     /=                                           PunctuationToken
                                          b![•.•]                                 ExpressionStatement, MacroInvocation
                                              .                                   PunctuationToken
                                                  b![•..•]                        ExpressionStatement, MacroInvocation
                                                      ..                          PunctuationToken
                                                           b![•...•]              ExpressionStatement, MacroInvocation
                                                               ...                PunctuationToken
                                                                     b![•..=•]    ExpressionStatement, MacroInvocation
                                                                         ..=      PunctuationToken                                        */
b![ = ] b![ == ] b![ >= ] b![ > ] b![ <= ] b![ < ] b![ *= ] b![ != ] b![ | ]                                                              /*
b![•=•]                                                                         ExpressionStatement, MacroInvocation
    =                                                                           PunctuationToken
        b![•==•]                                                                ExpressionStatement, MacroInvocation
            ==                                                                  PunctuationToken
                 b![•>=•]                                                       ExpressionStatement, MacroInvocation
                     >=                                                         PunctuationToken
                          b![•>•]                                               ExpressionStatement, MacroInvocation
                              >                                                 PunctuationToken
                                  b![•<=•]                                      ExpressionStatement, MacroInvocation
                                      <=                                        PunctuationToken
                                           b![•<•]                              ExpressionStatement, MacroInvocation
                                               <                                PunctuationToken
                                                   b![•*=•]                     ExpressionStatement, MacroInvocation
                                                       *=                       PunctuationToken
                                                            b![•!=•]            ExpressionStatement, MacroInvocation
                                                                !=              PunctuationToken
                                                                     b![•|•]    ExpressionStatement, MacroInvocation
                                                                         |      PunctuationToken                                          */
b![ |= ] b![ || ] b![ # ] b![ ? ] b![ -> ] b![ <- ] b![ % ] b![ %= ] b![ => ]                                                             /*
b![•|=•]                                                                         ExpressionStatement, MacroInvocation
    |=                                                                           PunctuationToken
         b![•||•]                                                                ExpressionStatement, MacroInvocation
             ||                                                                  PunctuationToken
                  b![•#•]                                                        ExpressionStatement, MacroInvocation
                      #                                                          PunctuationToken
                          b![•?•]                                                ExpressionStatement, MacroInvocation
                              ?                                                  PunctuationToken
                                  b![•->•]                                       ExpressionStatement, MacroInvocation
                                      ->                                         PunctuationToken
                                           b![•<-•]                              ExpressionStatement, MacroInvocation
                                               <                                 PunctuationToken
                                                -                                PunctuationToken
                                                    b![•%•]                      ExpressionStatement, MacroInvocation
                                                        %                        PunctuationToken
                                                            b![•%=•]             ExpressionStatement, MacroInvocation
                                                                %=               PunctuationToken
                                                                     b![•=>•]    ExpressionStatement, MacroInvocation
                                                                         =>      PunctuationToken                                         */
b![ ; ] b![ << ] b![ <<= ] b![ >> ] b![ >>= ] b![ * ] b![ - ] b![ -= ] b![ ~ ]                                                            /*
b![•;•]                                                                           ExpressionStatement, MacroInvocation
    ;                                                                             PunctuationToken
        b![•<<•]                                                                  ExpressionStatement, MacroInvocation
            <<                                                                    PunctuationToken
                 b![•<<=•]                                                        ExpressionStatement, MacroInvocation
                     <<=                                                          PunctuationToken
                           b![•>>•]                                               ExpressionStatement, MacroInvocation
                               >>                                                 PunctuationToken
                                    b![•>>=•]                                     ExpressionStatement, MacroInvocation
                                        >>=                                       PunctuationToken
                                              b![•*•]                             ExpressionStatement, MacroInvocation
                                                  *                               PunctuationToken
                                                      b![•-•]                     ExpressionStatement, MacroInvocation
                                                          -                       PunctuationToken
                                                              b![•-=•]            ExpressionStatement, MacroInvocation
                                                                  -=              PunctuationToken
                                                                       b![•~•]    ExpressionStatement, MacroInvocation
                                                                           ~      PunctuationToken                                        */

c!( + ) c!( += ) c!( & ) c!( && ) c!( &= ) c!( @ ) c!( ! ) c!( ^ ) c!( ^= )                                                               /*
c!(•+•)                                                                        ExpressionStatement, MacroInvocation
    +                                                                          PunctuationToken
        c!(•+=•)                                                               ExpressionStatement, MacroInvocation
            +=                                                                 PunctuationToken
                 c!(•&•)                                                       ExpressionStatement, MacroInvocation
                     &                                                         PunctuationToken
                         c!(•&&•)                                              ExpressionStatement, MacroInvocation
                             &&                                                PunctuationToken
                                  c!(•&=•)                                     ExpressionStatement, MacroInvocation
                                      &=                                       PunctuationToken
                                           c!(•@•)                             ExpressionStatement, MacroInvocation
                                               @                               PunctuationToken
                                                   c!(•!•)                     ExpressionStatement, MacroInvocation
                                                       !                       PunctuationToken
                                                           c!(•^•)             ExpressionStatement, MacroInvocation
                                                               ^               PunctuationToken
                                                                   c!(•^=•)    ExpressionStatement, MacroInvocation
                                                                       ^=      PunctuationToken                                           */
c!( : ) c!( :: ) c!( , ) c!( / ) c!( /= ) c!( . ) c!( .. ) c!( ... ) c!( ..= )                                                            /*
c!(•:•)                                                                           ExpressionStatement, MacroInvocation
    :                                                                             PunctuationToken
        c!(•::•)                                                                  ExpressionStatement, MacroInvocation
            ::                                                                    PunctuationToken
                 c!(•,•)                                                          ExpressionStatement, MacroInvocation
                     ,                                                            PunctuationToken
                         c!(•/•)                                                  ExpressionStatement, MacroInvocation
                             /                                                    PunctuationToken
                                 c!(•/=•)                                         ExpressionStatement, MacroInvocation
                                     /=                                           PunctuationToken
                                          c!(•.•)                                 ExpressionStatement, MacroInvocation
                                              .                                   PunctuationToken
                                                  c!(•..•)                        ExpressionStatement, MacroInvocation
                                                      ..                          PunctuationToken
                                                           c!(•...•)              ExpressionStatement, MacroInvocation
                                                               ...                PunctuationToken
                                                                     c!(•..=•)    ExpressionStatement, MacroInvocation
                                                                         ..=      PunctuationToken                                        */
c!( = ) c!( == ) c!( >= ) c!( > ) c!( <= ) c!( < ) c!( *= ) c!( != ) c!( | )                                                              /*
c!(•=•)                                                                         ExpressionStatement, MacroInvocation
    =                                                                           PunctuationToken
        c!(•==•)                                                                ExpressionStatement, MacroInvocation
            ==                                                                  PunctuationToken
                 c!(•>=•)                                                       ExpressionStatement, MacroInvocation
                     >=                                                         PunctuationToken
                          c!(•>•)                                               ExpressionStatement, MacroInvocation
                              >                                                 PunctuationToken
                                  c!(•<=•)                                      ExpressionStatement, MacroInvocation
                                      <=                                        PunctuationToken
                                           c!(•<•)                              ExpressionStatement, MacroInvocation
                                               <                                PunctuationToken
                                                   c!(•*=•)                     ExpressionStatement, MacroInvocation
                                                       *=                       PunctuationToken
                                                            c!(•!=•)            ExpressionStatement, MacroInvocation
                                                                !=              PunctuationToken
                                                                     c!(•|•)    ExpressionStatement, MacroInvocation
                                                                         |      PunctuationToken                                          */
c!( |= ) c!( || ) c!( # ) c!( ? ) c!( -> ) c!( <- ) c!( % ) c!( %= ) c!( => )                                                             /*
c!(•|=•)                                                                         ExpressionStatement, MacroInvocation
    |=                                                                           PunctuationToken
         c!(•||•)                                                                ExpressionStatement, MacroInvocation
             ||                                                                  PunctuationToken
                  c!(•#•)                                                        ExpressionStatement, MacroInvocation
                      #                                                          PunctuationToken
                          c!(•?•)                                                ExpressionStatement, MacroInvocation
                              ?                                                  PunctuationToken
                                  c!(•->•)                                       ExpressionStatement, MacroInvocation
                                      ->                                         PunctuationToken
                                           c!(•<-•)                              ExpressionStatement, MacroInvocation
                                               <                                 PunctuationToken
                                                -                                PunctuationToken
                                                    c!(•%•)                      ExpressionStatement, MacroInvocation
                                                        %                        PunctuationToken
                                                            c!(•%=•)             ExpressionStatement, MacroInvocation
                                                                %=               PunctuationToken
                                                                     c!(•=>•)    ExpressionStatement, MacroInvocation
                                                                         =>      PunctuationToken                                         */
c!( ; ) c!( << ) c!( <<= ) c!( >> ) c!( >>= ) c!( * ) c!( - ) c!( -= ) c!( ~ )                                                            /*
c!(•;•)                                                                           ExpressionStatement, MacroInvocation
    ;                                                                             PunctuationToken
        c!(•<<•)                                                                  ExpressionStatement, MacroInvocation
            <<                                                                    PunctuationToken
                 c!(•<<=•)                                                        ExpressionStatement, MacroInvocation
                     <<=                                                          PunctuationToken
                           c!(•>>•)                                               ExpressionStatement, MacroInvocation
                               >>                                                 PunctuationToken
                                    c!(•>>=•)                                     ExpressionStatement, MacroInvocation
                                        >>=                                       PunctuationToken
                                              c!(•*•)                             ExpressionStatement, MacroInvocation
                                                  *                               PunctuationToken
                                                      c!(•-•)                     ExpressionStatement, MacroInvocation
                                                          -                       PunctuationToken
                                                              c!(•-=•)            ExpressionStatement, MacroInvocation
                                                                  -=              PunctuationToken
                                                                       c!(•~•)    ExpressionStatement, MacroInvocation
                                                                           ~      PunctuationToken                                        */

macro_rules! x {                                                                                                                          /*
macro_rules!•x•{↲    <MacroRulesDeclaration>                                                                                              */
	($p:pat =>) => {};                                                                                                                    /*
    ($p:pat•=>)•=>•{}    MacroRuleDeclaration
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
            =>           PunctuationToken                                                                                                 */
    ($p:pat ,) => {};                                                                                                                     /*
    ($p:pat•,)•=>•{}     MacroRuleDeclaration
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
            ,            PunctuationToken                                                                                                 */
    ($p:pat =) => {};                                                                                                                     /*
    ($p:pat•=)•=>•{}     MacroRuleDeclaration
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
            =            PunctuationToken                                                                                                 */
    ($p:pat |) => {};                                                                                                                     /*
    ($p:pat•|)•=>•{}     MacroRuleDeclaration
     $p:pat              MacroParameterDeclaration
     $p                  McIdentifier
            |            PunctuationToken                                                                                                 */
    ($p:pat if) => {};                                                                                                                    /*
    ($p:pat•if)•=>•{}     MacroRuleDeclaration
     $p:pat               MacroParameterDeclaration
     $p                   McIdentifier                                                                                                    */
    ($p:pat in) => {};                                                                                                                    /*
    ($p:pat•in)•=>•{}     MacroRuleDeclaration
     $p:pat               MacroParameterDeclaration
     $p                   McIdentifier                                                                                                    */
	($e:expr =>) => {};                                                                                                                   /*
    ($e:expr•=>)•=>•{}    MacroRuleDeclaration
     $e:expr              MacroParameterDeclaration
     $e                   McIdentifier
             =>           PunctuationToken                                                                                                */
    ($e:expr ,) => {};                                                                                                                    /*
    ($e:expr•,)•=>•{}     MacroRuleDeclaration
     $e:expr              MacroParameterDeclaration
     $e                   McIdentifier
             ,            PunctuationToken                                                                                                */
    ($e:expr ;) => {};                                                                                                                    /*
    ($e:expr•;)•=>•{}     MacroRuleDeclaration
     $e:expr              MacroParameterDeclaration
     $e                   McIdentifier
             ;            PunctuationToken                                                                                                */
	($t:ty {}) => {};                                                                                                                     /*
    ($t:ty•{})•=>•{}    MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           {}           DelimGroup                                                                                                        */
    ($t:ty ,) => {};                                                                                                                      /*
    ($t:ty•,)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           ,            PunctuationToken                                                                                                  */
    ($t:ty =>) => {};                                                                                                                     /*
    ($t:ty•=>)•=>•{}     MacroRuleDeclaration
     $t:ty               MacroParameterDeclaration
     $t                  McIdentifier
           =>            PunctuationToken                                                                                                 */
    ($t:ty :) => {};                                                                                                                      /*
    ($t:ty•:)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           :            PunctuationToken                                                                                                  */
    ($t:ty =) => {};                                                                                                                      /*
    ($t:ty•=)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           =            PunctuationToken                                                                                                  */
    ($t:ty >) => {};                                                                                                                      /*
    ($t:ty•>)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           >            PunctuationToken                                                                                                  */
    ($t:ty ;) => {};                                                                                                                      /*
    ($t:ty•;)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           ;            PunctuationToken                                                                                                  */
    ($t:ty |) => {};                                                                                                                      /*
    ($t:ty•|)•=>•{}     MacroRuleDeclaration
     $t:ty              MacroParameterDeclaration
     $t                 McIdentifier
           |            PunctuationToken                                                                                                  */
    ($t:ty as) => {};                                                                                                                     /*
    ($t:ty•as)•=>•{}     MacroRuleDeclaration
     $t:ty               MacroParameterDeclaration
     $t                  McIdentifier                                                                                                     */
    ($t:ty where) => {};                                                                                                                  /*
    ($t:ty•where)•=>•{}     MacroRuleDeclaration
     $t:ty                  MacroParameterDeclaration
     $t                     McIdentifier                                                                                                  */
    ($t:ty []) => {};                                                                                                                     /*
    ($t:ty•[])•=>•{}     MacroRuleDeclaration
     $t:ty               MacroParameterDeclaration
     $t                  McIdentifier
           []            DelimGroup                                                                                                       */
    ($t:ty $b:block) => {};                                                                                                               /*
    ($t:ty•$b:block)•=>•{}     MacroRuleDeclaration
     $t:ty                     MacroParameterDeclaration
     $t                        McIdentifier
           $b:block            MacroParameterDeclaration
           $b                  McIdentifier                                                                                               */
	($s:stmt =>) => {};                                                                                                                   /*
    ($s:stmt•=>)•=>•{}    MacroRuleDeclaration
     $s:stmt              MacroParameterDeclaration
     $s                   McIdentifier
             =>           PunctuationToken                                                                                                */
    ($s:stmt ,) => {};                                                                                                                    /*
    ($s:stmt•,)•=>•{}     MacroRuleDeclaration
     $s:stmt              MacroParameterDeclaration
     $s                   McIdentifier
             ,            PunctuationToken                                                                                                */
    ($s:stmt ;) => {};                                                                                                                    /*
    ($s:stmt•;)•=>•{}     MacroRuleDeclaration
     $s:stmt              MacroParameterDeclaration
     $s                   McIdentifier
             ;            PunctuationToken                                                                                                */
	($p:path {}) => {};                                                                                                                   /*
    ($p:path•{})•=>•{}    MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             {}           DelimGroup                                                                                                      */
    ($p:path ,) => {};                                                                                                                    /*
    ($p:path•,)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             ,            PunctuationToken                                                                                                */
    ($p:path =>) => {};                                                                                                                   /*
    ($p:path•=>)•=>•{}     MacroRuleDeclaration
     $p:path               MacroParameterDeclaration
     $p                    McIdentifier
             =>            PunctuationToken                                                                                               */
    ($p:path :) => {};                                                                                                                    /*
    ($p:path•:)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             :            PunctuationToken                                                                                                */
    ($p:path =) => {};                                                                                                                    /*
    ($p:path•=)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             =            PunctuationToken                                                                                                */
    ($p:path >) => {};                                                                                                                    /*
    ($p:path•>)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             >            PunctuationToken                                                                                                */
    ($p:path ;) => {};                                                                                                                    /*
    ($p:path•;)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             ;            PunctuationToken                                                                                                */
    ($p:path |) => {};                                                                                                                    /*
    ($p:path•|)•=>•{}     MacroRuleDeclaration
     $p:path              MacroParameterDeclaration
     $p                   McIdentifier
             |            PunctuationToken                                                                                                */
    ($p:path as) => {};                                                                                                                   /*
    ($p:path•as)•=>•{}     MacroRuleDeclaration
     $p:path               MacroParameterDeclaration
     $p                    McIdentifier                                                                                                   */
    ($p:path where) => {};                                                                                                                /*
    ($p:path•where)•=>•{}     MacroRuleDeclaration
     $p:path                  MacroParameterDeclaration
     $p                       McIdentifier                                                                                                */
    ($p:path []) => {};                                                                                                                   /*
    ($p:path•[])•=>•{}     MacroRuleDeclaration
     $p:path               MacroParameterDeclaration
     $p                    McIdentifier
             []            DelimGroup                                                                                                     */
    ($p:path $b:block) => {};                                                                                                             /*
    ($p:path•$b:block)•=>•{}     MacroRuleDeclaration
     $p:path                     MacroParameterDeclaration
     $p                          McIdentifier
             $b:block            MacroParameterDeclaration
             $b                  McIdentifier                                                                                             */
	($b:block ()) => {};                                                                                                                  /*
    ($b:block•())•=>•{}    MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              ()           DelimGroup                                                                                                     */
    ($b:block []) => {};                                                                                                                  /*
    ($b:block•[])•=>•{}     MacroRuleDeclaration
     $b:block               MacroParameterDeclaration
     $b                     McIdentifier
              []            DelimGroup                                                                                                    */
    ($b:block {}) => {};                                                                                                                  /*
    ($b:block•{})•=>•{}     MacroRuleDeclaration
     $b:block               MacroParameterDeclaration
     $b                     McIdentifier
              {}            DelimGroup                                                                                                    */
    ($b:block ,) => {};                                                                                                                   /*
    ($b:block•,)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              ,            PunctuationToken                                                                                               */
    ($b:block =>) => {};                                                                                                                  /*
    ($b:block•=>)•=>•{}     MacroRuleDeclaration
     $b:block               MacroParameterDeclaration
     $b                     McIdentifier
              =>            PunctuationToken                                                                                              */
    ($b:block :) => {};                                                                                                                   /*
    ($b:block•:)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              :            PunctuationToken                                                                                               */
    ($b:block =) => {};                                                                                                                   /*
    ($b:block•=)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              =            PunctuationToken                                                                                               */
    ($b:block >) => {};                                                                                                                   /*
    ($b:block•>)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              >            PunctuationToken                                                                                               */
    ($b:block ;) => {};                                                                                                                   /*
    ($b:block•;)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              ;            PunctuationToken                                                                                               */
    ($b:block |) => {};                                                                                                                   /*
    ($b:block•|)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              |            PunctuationToken                                                                                               */
    ($b:block +) => {};                                                                                                                   /*
    ($b:block•+)•=>•{}     MacroRuleDeclaration
     $b:block              MacroParameterDeclaration
     $b                    McIdentifier
              +            PunctuationToken                                                                                               */
    ($b:block ident) => {};                                                                                                               /*
    ($b:block•ident)•=>•{}     MacroRuleDeclaration
     $b:block                  MacroParameterDeclaration
     $b                        McIdentifier                                                                                               */
    ($b:block $p:pat) => {};                                                                                                              /*
    ($b:block•$p:pat)•=>•{}     MacroRuleDeclaration
     $b:block                   MacroParameterDeclaration
     $b                         McIdentifier
              $p:pat            MacroParameterDeclaration
              $p                McIdentifier                                                                                              */
    ($b:block $e:expr) => {};                                                                                                             /*
    ($b:block•$e:expr)•=>•{}     MacroRuleDeclaration
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $e:expr            MacroParameterDeclaration
              $e                 McIdentifier                                                                                             */
    ($b:block $t:ty) => {};                                                                                                               /*
    ($b:block•$t:ty)•=>•{}     MacroRuleDeclaration
     $b:block                  MacroParameterDeclaration
     $b                        McIdentifier
              $t:ty            MacroParameterDeclaration
              $t               McIdentifier                                                                                               */
    ($b:block $s:stmt) => {};                                                                                                             /*
    ($b:block•$s:stmt)•=>•{}     MacroRuleDeclaration
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $s:stmt            MacroParameterDeclaration
              $s                 McIdentifier                                                                                             */
    ($b:block $p:path) => {};                                                                                                             /*
    ($b:block•$p:path)•=>•{}     MacroRuleDeclaration
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $p:path            MacroParameterDeclaration
              $p                 McIdentifier                                                                                             */
    ($b:block $c:block) => {};                                                                                                            /*
    ($b:block•$c:block)•=>•{}     MacroRuleDeclaration
     $b:block                     MacroParameterDeclaration
     $b                           McIdentifier
              $c:block            MacroParameterDeclaration
              $c                  McIdentifier                                                                                            */
    ($b:block $i:ident) => {};                                                                                                            /*
    ($b:block•$i:ident)•=>•{}     MacroRuleDeclaration
     $b:block                     MacroParameterDeclaration
     $b                           McIdentifier
              $i:ident            MacroParameterDeclaration
              $i                  McIdentifier                                                                                            */
    ($b:block $t:tt) => {};                                                                                                               /*
    ($b:block•$t:tt)•=>•{}     MacroRuleDeclaration
     $b:block                  MacroParameterDeclaration
     $b                        McIdentifier
              $t:tt            MacroParameterDeclaration
              $t               McIdentifier                                                                                               */
    ($b:block $i:item) => {};                                                                                                             /*
    ($b:block•$i:item)•=>•{}     MacroRuleDeclaration
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $i:item            MacroParameterDeclaration
              $i                 McIdentifier                                                                                             */
    ($b:block $m:meta) => {};                                                                                                             /*
    ($b:block•$m:meta)•=>•{}     MacroRuleDeclaration
     $b:block                    MacroParameterDeclaration
     $b                          McIdentifier
              $m:meta            MacroParameterDeclaration
              $m                 McIdentifier                                                                                             */
	($i:ident ()) => {};                                                                                                                  /*
    ($i:ident•())•=>•{}    MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              ()           DelimGroup                                                                                                     */
    ($i:ident []) => {};                                                                                                                  /*
    ($i:ident•[])•=>•{}     MacroRuleDeclaration
     $i:ident               MacroParameterDeclaration
     $i                     McIdentifier
              []            DelimGroup                                                                                                    */
    ($i:ident {}) => {};                                                                                                                  /*
    ($i:ident•{})•=>•{}     MacroRuleDeclaration
     $i:ident               MacroParameterDeclaration
     $i                     McIdentifier
              {}            DelimGroup                                                                                                    */
    ($i:ident ,) => {};                                                                                                                   /*
    ($i:ident•,)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              ,            PunctuationToken                                                                                               */
    ($i:ident =>) => {};                                                                                                                  /*
    ($i:ident•=>)•=>•{}     MacroRuleDeclaration
     $i:ident               MacroParameterDeclaration
     $i                     McIdentifier
              =>            PunctuationToken                                                                                              */
    ($i:ident :) => {};                                                                                                                   /*
    ($i:ident•:)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              :            PunctuationToken                                                                                               */
    ($i:ident =) => {};                                                                                                                   /*
    ($i:ident•=)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              =            PunctuationToken                                                                                               */
    ($i:ident >) => {};                                                                                                                   /*
    ($i:ident•>)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              >            PunctuationToken                                                                                               */
    ($i:ident ;) => {};                                                                                                                   /*
    ($i:ident•;)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              ;            PunctuationToken                                                                                               */
    ($i:ident |) => {};                                                                                                                   /*
    ($i:ident•|)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              |            PunctuationToken                                                                                               */
    ($i:ident +) => {};                                                                                                                   /*
    ($i:ident•+)•=>•{}     MacroRuleDeclaration
     $i:ident              MacroParameterDeclaration
     $i                    McIdentifier
              +            PunctuationToken                                                                                               */
    ($i:ident ident) => {};                                                                                                               /*
    ($i:ident•ident)•=>•{}     MacroRuleDeclaration
     $i:ident                  MacroParameterDeclaration
     $i                        McIdentifier                                                                                               */
    ($i:ident $p:pat) => {};                                                                                                              /*
    ($i:ident•$p:pat)•=>•{}     MacroRuleDeclaration
     $i:ident                   MacroParameterDeclaration
     $i                         McIdentifier
              $p:pat            MacroParameterDeclaration
              $p                McIdentifier                                                                                              */
    ($i:ident $e:expr) => {};                                                                                                             /*
    ($i:ident•$e:expr)•=>•{}     MacroRuleDeclaration
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $e:expr            MacroParameterDeclaration
              $e                 McIdentifier                                                                                             */
    ($i:ident $t:ty) => {};                                                                                                               /*
    ($i:ident•$t:ty)•=>•{}     MacroRuleDeclaration
     $i:ident                  MacroParameterDeclaration
     $i                        McIdentifier
              $t:ty            MacroParameterDeclaration
              $t               McIdentifier                                                                                               */
    ($i:ident $s:stmt) => {};                                                                                                             /*
    ($i:ident•$s:stmt)•=>•{}     MacroRuleDeclaration
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $s:stmt            MacroParameterDeclaration
              $s                 McIdentifier                                                                                             */
    ($i:ident $p:path) => {};                                                                                                             /*
    ($i:ident•$p:path)•=>•{}     MacroRuleDeclaration
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $p:path            MacroParameterDeclaration
              $p                 McIdentifier                                                                                             */
    ($i:ident $b:block) => {};                                                                                                            /*
    ($i:ident•$b:block)•=>•{}     MacroRuleDeclaration
     $i:ident                     MacroParameterDeclaration
     $i                           McIdentifier
              $b:block            MacroParameterDeclaration
              $b                  McIdentifier                                                                                            */
    ($i:ident $j:ident) => {};                                                                                                            /*
    ($i:ident•$j:ident)•=>•{}     MacroRuleDeclaration
     $i:ident                     MacroParameterDeclaration
     $i                           McIdentifier
              $j:ident            MacroParameterDeclaration
              $j                  McIdentifier                                                                                            */
    ($i:ident $t:tt) => {};                                                                                                               /*
    ($i:ident•$t:tt)•=>•{}     MacroRuleDeclaration
     $i:ident                  MacroParameterDeclaration
     $i                        McIdentifier
              $t:tt            MacroParameterDeclaration
              $t               McIdentifier                                                                                               */
    ($i:ident $j:item) => {};                                                                                                             /*
    ($i:ident•$j:item)•=>•{}     MacroRuleDeclaration
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $j:item            MacroParameterDeclaration
              $j                 McIdentifier                                                                                             */
    ($i:ident $m:meta) => {};                                                                                                             /*
    ($i:ident•$m:meta)•=>•{}     MacroRuleDeclaration
     $i:ident                    MacroParameterDeclaration
     $i                          McIdentifier
              $m:meta            MacroParameterDeclaration
              $m                 McIdentifier                                                                                             */
	($t:tt ()) => {};                                                                                                                     /*
    ($t:tt•())•=>•{}    MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           ()           DelimGroup                                                                                                        */
    ($t:tt []) => {};                                                                                                                     /*
    ($t:tt•[])•=>•{}     MacroRuleDeclaration
     $t:tt               MacroParameterDeclaration
     $t                  McIdentifier
           []            DelimGroup                                                                                                       */
    ($t:tt {}) => {};                                                                                                                     /*
    ($t:tt•{})•=>•{}     MacroRuleDeclaration
     $t:tt               MacroParameterDeclaration
     $t                  McIdentifier
           {}            DelimGroup                                                                                                       */
    ($t:tt ,) => {};                                                                                                                      /*
    ($t:tt•,)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           ,            PunctuationToken                                                                                                  */
    ($t:tt =>) => {};                                                                                                                     /*
    ($t:tt•=>)•=>•{}     MacroRuleDeclaration
     $t:tt               MacroParameterDeclaration
     $t                  McIdentifier
           =>            PunctuationToken                                                                                                 */
    ($t:tt :) => {};                                                                                                                      /*
    ($t:tt•:)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           :            PunctuationToken                                                                                                  */
    ($t:tt =) => {};                                                                                                                      /*
    ($t:tt•=)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           =            PunctuationToken                                                                                                  */
    ($t:tt >) => {};                                                                                                                      /*
    ($t:tt•>)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           >            PunctuationToken                                                                                                  */
    ($t:tt ;) => {};                                                                                                                      /*
    ($t:tt•;)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           ;            PunctuationToken                                                                                                  */
    ($t:tt |) => {};                                                                                                                      /*
    ($t:tt•|)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           |            PunctuationToken                                                                                                  */
    ($t:tt +) => {};                                                                                                                      /*
    ($t:tt•+)•=>•{}     MacroRuleDeclaration
     $t:tt              MacroParameterDeclaration
     $t                 McIdentifier
           +            PunctuationToken                                                                                                  */
    ($t:tt ident) => {};                                                                                                                  /*
    ($t:tt•ident)•=>•{}     MacroRuleDeclaration
     $t:tt                  MacroParameterDeclaration
     $t                     McIdentifier                                                                                                  */
    ($t:tt $p:pat) => {};                                                                                                                 /*
    ($t:tt•$p:pat)•=>•{}     MacroRuleDeclaration
     $t:tt                   MacroParameterDeclaration
     $t                      McIdentifier
           $p:pat            MacroParameterDeclaration
           $p                McIdentifier                                                                                                 */
    ($t:tt $e:expr) => {};                                                                                                                /*
    ($t:tt•$e:expr)•=>•{}     MacroRuleDeclaration
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $e:expr            MacroParameterDeclaration
           $e                 McIdentifier                                                                                                */
    ($t:tt $v:ty) => {};                                                                                                                  /*
    ($t:tt•$v:ty)•=>•{}     MacroRuleDeclaration
     $t:tt                  MacroParameterDeclaration
     $t                     McIdentifier
           $v:ty            MacroParameterDeclaration
           $v               McIdentifier                                                                                                  */
    ($t:tt $s:stmt) => {};                                                                                                                /*
    ($t:tt•$s:stmt)•=>•{}     MacroRuleDeclaration
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $s:stmt            MacroParameterDeclaration
           $s                 McIdentifier                                                                                                */
    ($t:tt $p:path) => {};                                                                                                                /*
    ($t:tt•$p:path)•=>•{}     MacroRuleDeclaration
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $p:path            MacroParameterDeclaration
           $p                 McIdentifier                                                                                                */
    ($t:tt $b:block) => {};                                                                                                               /*
    ($t:tt•$b:block)•=>•{}     MacroRuleDeclaration
     $t:tt                     MacroParameterDeclaration
     $t                        McIdentifier
           $b:block            MacroParameterDeclaration
           $b                  McIdentifier                                                                                               */
    ($t:tt $i:ident) => {};                                                                                                               /*
    ($t:tt•$i:ident)•=>•{}     MacroRuleDeclaration
     $t:tt                     MacroParameterDeclaration
     $t                        McIdentifier
           $i:ident            MacroParameterDeclaration
           $i                  McIdentifier                                                                                               */
    ($t:tt $v:tt) => {};                                                                                                                  /*
    ($t:tt•$v:tt)•=>•{}     MacroRuleDeclaration
     $t:tt                  MacroParameterDeclaration
     $t                     McIdentifier
           $v:tt            MacroParameterDeclaration
           $v               McIdentifier                                                                                                  */
    ($t:tt $i:item) => {};                                                                                                                /*
    ($t:tt•$i:item)•=>•{}     MacroRuleDeclaration
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $i:item            MacroParameterDeclaration
           $i                 McIdentifier                                                                                                */
    ($t:tt $m:meta) => {};                                                                                                                /*
    ($t:tt•$m:meta)•=>•{}     MacroRuleDeclaration
     $t:tt                    MacroParameterDeclaration
     $t                       McIdentifier
           $m:meta            MacroParameterDeclaration
           $m                 McIdentifier                                                                                                */
	($i:item ()) => {};                                                                                                                   /*
    ($i:item•())•=>•{}    MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             ()           DelimGroup                                                                                                      */
    ($i:item []) => {};                                                                                                                   /*
    ($i:item•[])•=>•{}     MacroRuleDeclaration
     $i:item               MacroParameterDeclaration
     $i                    McIdentifier
             []            DelimGroup                                                                                                     */
    ($i:item {}) => {};                                                                                                                   /*
    ($i:item•{})•=>•{}     MacroRuleDeclaration
     $i:item               MacroParameterDeclaration
     $i                    McIdentifier
             {}            DelimGroup                                                                                                     */
    ($i:item ,) => {};                                                                                                                    /*
    ($i:item•,)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             ,            PunctuationToken                                                                                                */
    ($i:item =>) => {};                                                                                                                   /*
    ($i:item•=>)•=>•{}     MacroRuleDeclaration
     $i:item               MacroParameterDeclaration
     $i                    McIdentifier
             =>            PunctuationToken                                                                                               */
    ($i:item :) => {};                                                                                                                    /*
    ($i:item•:)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             :            PunctuationToken                                                                                                */
    ($i:item =) => {};                                                                                                                    /*
    ($i:item•=)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             =            PunctuationToken                                                                                                */
    ($i:item >) => {};                                                                                                                    /*
    ($i:item•>)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             >            PunctuationToken                                                                                                */
    ($i:item ;) => {};                                                                                                                    /*
    ($i:item•;)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             ;            PunctuationToken                                                                                                */
    ($i:item |) => {};                                                                                                                    /*
    ($i:item•|)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             |            PunctuationToken                                                                                                */
    ($i:item +) => {};                                                                                                                    /*
    ($i:item•+)•=>•{}     MacroRuleDeclaration
     $i:item              MacroParameterDeclaration
     $i                   McIdentifier
             +            PunctuationToken                                                                                                */
    ($i:item ident) => {};                                                                                                                /*
    ($i:item•ident)•=>•{}     MacroRuleDeclaration
     $i:item                  MacroParameterDeclaration
     $i                       McIdentifier                                                                                                */
    ($i:item $p:pat) => {};                                                                                                               /*
    ($i:item•$p:pat)•=>•{}     MacroRuleDeclaration
     $i:item                   MacroParameterDeclaration
     $i                        McIdentifier
             $p:pat            MacroParameterDeclaration
             $p                McIdentifier                                                                                               */
    ($i:item $e:expr) => {};                                                                                                              /*
    ($i:item•$e:expr)•=>•{}     MacroRuleDeclaration
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $e:expr            MacroParameterDeclaration
             $e                 McIdentifier                                                                                              */
    ($i:item $t:ty) => {};                                                                                                                /*
    ($i:item•$t:ty)•=>•{}     MacroRuleDeclaration
     $i:item                  MacroParameterDeclaration
     $i                       McIdentifier
             $t:ty            MacroParameterDeclaration
             $t               McIdentifier                                                                                                */
    ($i:item $s:stmt) => {};                                                                                                              /*
    ($i:item•$s:stmt)•=>•{}     MacroRuleDeclaration
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $s:stmt            MacroParameterDeclaration
             $s                 McIdentifier                                                                                              */
    ($i:item $p:path) => {};                                                                                                              /*
    ($i:item•$p:path)•=>•{}     MacroRuleDeclaration
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $p:path            MacroParameterDeclaration
             $p                 McIdentifier                                                                                              */
    ($i:item $b:block) => {};                                                                                                             /*
    ($i:item•$b:block)•=>•{}     MacroRuleDeclaration
     $i:item                     MacroParameterDeclaration
     $i                          McIdentifier
             $b:block            MacroParameterDeclaration
             $b                  McIdentifier                                                                                             */
    ($i:item $j:ident) => {};                                                                                                             /*
    ($i:item•$j:ident)•=>•{}     MacroRuleDeclaration
     $i:item                     MacroParameterDeclaration
     $i                          McIdentifier
             $j:ident            MacroParameterDeclaration
             $j                  McIdentifier                                                                                             */
    ($i:item $t:tt) => {};                                                                                                                /*
    ($i:item•$t:tt)•=>•{}     MacroRuleDeclaration
     $i:item                  MacroParameterDeclaration
     $i                       McIdentifier
             $t:tt            MacroParameterDeclaration
             $t               McIdentifier                                                                                                */
    ($i:item $j:item) => {};                                                                                                              /*
    ($i:item•$j:item)•=>•{}     MacroRuleDeclaration
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $j:item            MacroParameterDeclaration
             $j                 McIdentifier                                                                                              */
    ($i:item $m:meta) => {};                                                                                                              /*
    ($i:item•$m:meta)•=>•{}     MacroRuleDeclaration
     $i:item                    MacroParameterDeclaration
     $i                         McIdentifier
             $m:meta            MacroParameterDeclaration
             $m                 McIdentifier                                                                                              */
	($m:meta ()) => {};                                                                                                                   /*
    ($m:meta•())•=>•{}    MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             ()           DelimGroup                                                                                                      */
    ($m:meta []) => {};                                                                                                                   /*
    ($m:meta•[])•=>•{}     MacroRuleDeclaration
     $m:meta               MacroParameterDeclaration
     $m                    McIdentifier
             []            DelimGroup                                                                                                     */
    ($m:meta {}) => {};                                                                                                                   /*
    ($m:meta•{})•=>•{}     MacroRuleDeclaration
     $m:meta               MacroParameterDeclaration
     $m                    McIdentifier
             {}            DelimGroup                                                                                                     */
    ($m:meta ,) => {};                                                                                                                    /*
    ($m:meta•,)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             ,            PunctuationToken                                                                                                */
    ($m:meta =>) => {};                                                                                                                   /*
    ($m:meta•=>)•=>•{}     MacroRuleDeclaration
     $m:meta               MacroParameterDeclaration
     $m                    McIdentifier
             =>            PunctuationToken                                                                                               */
    ($m:meta :) => {};                                                                                                                    /*
    ($m:meta•:)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             :            PunctuationToken                                                                                                */
    ($m:meta =) => {};                                                                                                                    /*
    ($m:meta•=)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             =            PunctuationToken                                                                                                */
    ($m:meta >) => {};                                                                                                                    /*
    ($m:meta•>)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             >            PunctuationToken                                                                                                */
    ($m:meta ;) => {};                                                                                                                    /*
    ($m:meta•;)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             ;            PunctuationToken                                                                                                */
    ($m:meta |) => {};                                                                                                                    /*
    ($m:meta•|)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             |            PunctuationToken                                                                                                */
    ($m:meta +) => {};                                                                                                                    /*
    ($m:meta•+)•=>•{}     MacroRuleDeclaration
     $m:meta              MacroParameterDeclaration
     $m                   McIdentifier
             +            PunctuationToken                                                                                                */
    ($m:meta ident) => {};                                                                                                                /*
    ($m:meta•ident)•=>•{}     MacroRuleDeclaration
     $m:meta                  MacroParameterDeclaration
     $m                       McIdentifier                                                                                                */
    ($m:meta $p:pat) => {};                                                                                                               /*
    ($m:meta•$p:pat)•=>•{}     MacroRuleDeclaration
     $m:meta                   MacroParameterDeclaration
     $m                        McIdentifier
             $p:pat            MacroParameterDeclaration
             $p                McIdentifier                                                                                               */
    ($m:meta $e:expr) => {};                                                                                                              /*
    ($m:meta•$e:expr)•=>•{}     MacroRuleDeclaration
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $e:expr            MacroParameterDeclaration
             $e                 McIdentifier                                                                                              */
    ($m:meta $t:ty) => {};                                                                                                                /*
    ($m:meta•$t:ty)•=>•{}     MacroRuleDeclaration
     $m:meta                  MacroParameterDeclaration
     $m                       McIdentifier
             $t:ty            MacroParameterDeclaration
             $t               McIdentifier                                                                                                */
    ($m:meta $s:stmt) => {};                                                                                                              /*
    ($m:meta•$s:stmt)•=>•{}     MacroRuleDeclaration
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $s:stmt            MacroParameterDeclaration
             $s                 McIdentifier                                                                                              */
    ($m:meta $p:path) => {};                                                                                                              /*
    ($m:meta•$p:path)•=>•{}     MacroRuleDeclaration
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $p:path            MacroParameterDeclaration
             $p                 McIdentifier                                                                                              */
    ($m:meta $b:block) => {};                                                                                                             /*
    ($m:meta•$b:block)•=>•{}     MacroRuleDeclaration
     $m:meta                     MacroParameterDeclaration
     $m                          McIdentifier
             $b:block            MacroParameterDeclaration
             $b                  McIdentifier                                                                                             */
    ($m:meta $i:ident) => {};                                                                                                             /*
    ($m:meta•$i:ident)•=>•{}     MacroRuleDeclaration
     $m:meta                     MacroParameterDeclaration
     $m                          McIdentifier
             $i:ident            MacroParameterDeclaration
             $i                  McIdentifier                                                                                             */
    ($m:meta $t:tt) => {};                                                                                                                /*
    ($m:meta•$t:tt)•=>•{}     MacroRuleDeclaration
     $m:meta                  MacroParameterDeclaration
     $m                       McIdentifier
             $t:tt            MacroParameterDeclaration
             $t               McIdentifier                                                                                                */
    ($m:meta $i:item) => {};                                                                                                              /*
    ($m:meta•$i:item)•=>•{}     MacroRuleDeclaration
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $i:item            MacroParameterDeclaration
             $i                 McIdentifier                                                                                              */
    ($m:meta $n:meta) => {};                                                                                                              /*
    ($m:meta•$n:meta)•=>•{}     MacroRuleDeclaration
     $m:meta                    MacroParameterDeclaration
     $m                         McIdentifier
             $n:meta            MacroParameterDeclaration
             $n                 McIdentifier                                                                                              */
	($ty:ty <) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
                                                                                                                                          /*
    ($ty:ty•<)•=>•()                                                                          MacroRuleDeclaration
     $ty:ty                                                                                   MacroParameterDeclaration
     $ty                                                                                      McIdentifier
            <                                                                                 PunctuationToken
                      //~•ERROR•`$ty:ty`•is•followed•by•`<`,•which•is•not•allowed•for•`ty`    Comment                                     */
    ($ty:ty < foo ,) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
                                                                                                                                          /*
    ($ty:ty•<•foo•,)•=>•()                                                                          MacroRuleDeclaration
     $ty:ty                                                                                         MacroParameterDeclaration
     $ty                                                                                            McIdentifier
            <                                                                                       PunctuationToken
                  ,                                                                                 PunctuationToken
                            //~•ERROR•`$ty:ty`•is•followed•by•`<`,•which•is•not•allowed•for•`ty`    Comment                               */
    ($ty:ty , ) => ();                                                                                                                    /*
    ($ty:ty•,•)•=>•()     MacroRuleDeclaration
     $ty:ty               MacroParameterDeclaration
     $ty                  McIdentifier
            ,             PunctuationToken                                                                                                */
    ( ( $ty:ty ) ) => ();                                                                                                                 /*
    (•(•$ty:ty•)•)•=>•()     MacroRuleDeclaration
      (•$ty:ty•)             DelimGroup
        $ty:ty               MacroParameterDeclaration
        $ty                  McIdentifier                                                                                                 */
    ( { $ty:ty } ) => ();                                                                                                                 /*
    (•{•$ty:ty•}•)•=>•()     MacroRuleDeclaration
      {•$ty:ty•}             DelimGroup
        $ty:ty               MacroParameterDeclaration
        $ty                  McIdentifier                                                                                                 */
    ( [ $ty:ty ] ) => ();                                                                                                                 /*
    (•[•$ty:ty•]•)•=>•()     MacroRuleDeclaration
      [•$ty:ty•]             DelimGroup
        $ty:ty               MacroParameterDeclaration
        $ty                  McIdentifier                                                                                                 */
    ($bl:block < ) => ();                                                                                                                 /*
    ($bl:block•<•)•=>•()     MacroRuleDeclaration
     $bl:block               MacroParameterDeclaration
     $bl                     McIdentifier
               <             PunctuationToken                                                                                             */
    ($pa:pat >) => (); //~ ERROR `$pa:pat` is followed by `>`, which is not allowed for `pat`
                                                                                                                                          /*
    ($pa:pat•>)•=>•()                                                                            MacroRuleDeclaration
     $pa:pat                                                                                     MacroParameterDeclaration
     $pa                                                                                         McIdentifier
             >                                                                                   PunctuationToken
                       //~•ERROR•`$pa:pat`•is•followed•by•`>`,•which•is•not•allowed•for•`pat`    Comment                                  */
    ($pa:pat , ) => ();                                                                                                                   /*
    ($pa:pat•,•)•=>•()     MacroRuleDeclaration
     $pa:pat               MacroParameterDeclaration
     $pa                   McIdentifier
             ,             PunctuationToken                                                                                               */
    ($pa:pat $pb:pat $ty:ty ,) => ();                                                                                                     /*
    ($pa:pat•$pb:pat•$ty:ty•,)•=>•()     MacroRuleDeclaration
     $pa:pat                             MacroParameterDeclaration
     $pa                                 McIdentifier
             $pb:pat                     MacroParameterDeclaration
             $pb                         McIdentifier
                     $ty:ty              MacroParameterDeclaration
                     $ty                 McIdentifier
                            ,            PunctuationToken                                                                                 */
    //~^ ERROR `$pa:pat` is followed by `$pb:pat`, which is not allowed
    //~^•ERROR•`$pa:pat`•is•followed•by•`$pb:pat`,•which•is•not•allowed    Comment
    //~^^ ERROR `$pb:pat` is followed by `$ty:ty`, which is not allowed
    //~^^•ERROR•`$pb:pat`•is•followed•by•`$ty:ty`,•which•is•not•allowed    Comment
    ($($ty:ty)* -) => (); //~ ERROR `$ty:ty` is followed by `-`
                                                                                                                                          /*
    ($($ty:ty)*•-)•=>•()                                           MacroRuleDeclaration
     $($ty:ty)*                                                    MacroGroup
       $ty:ty                                                      MacroParameterDeclaration
       $ty                                                         McIdentifier
                -                                                  PunctuationToken
                          //~•ERROR•`$ty:ty`•is•followed•by•`-`    Comment                                                                */
    ($($a:ty, $b:ty)* -) => (); //~ ERROR `$b:ty` is followed by `-`
                                                                                                                                          /*
    ($($a:ty,•$b:ty)*•-)•=>•()                                          MacroRuleDeclaration
     $($a:ty,•$b:ty)*                                                   MacroGroup
       $a:ty                                                            MacroParameterDeclaration
       $a                                                               McIdentifier
            ,                                                           PunctuationToken
              $b:ty                                                     MacroParameterDeclaration
              $b                                                        McIdentifier
                      -                                                 PunctuationToken
                                //~•ERROR•`$b:ty`•is•followed•by•`-`    Comment                                                           */
    ($($ty:ty)-+) => (); //~ ERROR `$ty:ty` is followed by `-`, which is not allowed for `ty`
                                                                                                                                          /*
    ($($ty:ty)-+)•=>•()                                                                          MacroRuleDeclaration
     $($ty:ty)-+                                                                                 MacroGroup
       $ty:ty                                                                                    MacroParameterDeclaration
       $ty                                                                                       McIdentifier
              -                                                                                  PunctuationToken
                         //~•ERROR•`$ty:ty`•is•followed•by•`-`,•which•is•not•allowed•for•`ty`    Comment                                  */
    ( $($a:expr)* $($b:tt)* ) => { };                                                                                                     /*
    (•$($a:expr)*•$($b:tt)*•)•=>•{•}     MacroRuleDeclaration
      $($a:expr)*                        MacroGroup
        $a:expr                          MacroParameterDeclaration
        $a                               McIdentifier
                  $($b:tt)*              MacroGroup
                    $b:tt                MacroParameterDeclaration
                    $b                   McIdentifier                                                                                     */
    //~^ ERROR `$a:expr` is followed by `$b:tt`, which is not allowed for `expr` fragments
    //~^•ERROR•`$a:expr`•is•followed•by•`$b:tt`,•which•is•not•allowed•for•`expr`•fragments    Comment
}                                                                                                                                         /*
}    </MacroRulesDeclaration>                                                                                                             */

#![rustc_dummy("hi", 1, 2, 1.012, pi = 3.14, bye, name("John"))]                                                                          /*
#![rustc_dummy("hi",•1,•2,•1.012,•pi•=•3.14,•bye,•name("John"))]    Attribute
              ("hi",•1,•2,•1.012,•pi•=•3.14,•bye,•name("John"))     DelimGroup
               "hi"                                                 Literal
                   ,                                                PunctuationToken
                     1                                              Literal
                      ,                                             PunctuationToken
                        2                                           Literal
                         ,                                          PunctuationToken
                           1.012                                    Literal
                                ,                                   PunctuationToken
                                     =                              PunctuationToken
                                       3.14                         Literal
                                           ,                        PunctuationToken
                                                ,                   PunctuationToken
                                                      ("John")      DelimGroup
                                                       "John"       Literal                                                               */
#[rustfmt::r#final(final)]                                                                                                                /*
#[rustfmt::r#final(final)]↲    <ExpressionStatement>
#[rustfmt::r#final(final)]     Attribute
         ::                    PunctuationToken
                  (final)      DelimGroup                                                                                                 */

lexes!{a #foo}                                                                                                                            /*
lexes!{a•#foo}    </ExpressionStatement>, MacroInvocation
         #        PunctuationToken                                                                                                        */
lexes!{continue 'foo}                                                                                                                     /*
lexes!{continue•'foo}    ExpressionStatement, MacroInvocation
                'foo     LtIdentifier                                                                                                     */
lexes!{match "..." {}}                                                                                                                    /*
lexes!{match•"..."•{}}    ExpressionStatement, MacroInvocation
             "..."        Literal
                   {}     DelimGroup                                                                                                      */
lexes!{r#let#foo} // Identifier<"r#let"; raw: true> PunctuationToken<#> Identifier<"foo">
                                                                                                                                          /*
lexes!{r#let#foo}                                                                            ExpressionStatement, MacroInvocation
            #                                                                                PunctuationToken
                  //•Identifier<"r#let";•raw:•true>•PunctuationToken<#>•Identifier<"foo">    Comment                                      */

fn f() {                                                                                                                                  /*
fn•f()•{↲    <FunctionDeclaration>                                                                                                        */
    unsafe {                                                                                                                              /*
    unsafe•{↲    <ExpressionStatement>, <BlockExpression>                                                                                 */
        asm!("");                                                                                                                         /*
        asm!("");    ExpressionStatement
        asm!("")     MacroInvocation
             ""      Literal                                                                                                              */
        asm!("", options());                                                                                                              /*
        asm!("",•options());    ExpressionStatement
        asm!("",•options())     MacroInvocation
             ""                 Literal
               ,                PunctuationToken
                        ()      DelimGroup                                                                                                */
        asm!("", options(nostack, nomem));                                                                                                /*
        asm!("",•options(nostack,•nomem));    ExpressionStatement
        asm!("",•options(nostack,•nomem))     MacroInvocation
             ""                               Literal
               ,                              PunctuationToken
                        (nostack,•nomem)      DelimGroup
                                ,             PunctuationToken                                                                            */
        asm!("{}", in(reg) 4);                                                                                                            /*
        asm!("{}",•in(reg)•4);    ExpressionStatement
        asm!("{}",•in(reg)•4)     MacroInvocation
             "{}"                 Literal
                 ,                PunctuationToken
                     (reg)        DelimGroup
                           4      Literal                                                                                                 */
        asm!("{0}", out(reg) a);                                                                                                          /*
        asm!("{0}",•out(reg)•a);    ExpressionStatement
        asm!("{0}",•out(reg)•a)     MacroInvocation
             "{0}"                  Literal
                  ,                 PunctuationToken
                       (reg)        DelimGroup                                                                                            */
        asm!("{name}", name = inout(reg) b);                                                                                              /*
        asm!("{name}",•name•=•inout(reg)•b);    ExpressionStatement
        asm!("{name}",•name•=•inout(reg)•b)     MacroInvocation
             "{name}"                           Literal
                     ,                          PunctuationToken
                            =                   PunctuationToken
                                   (reg)        DelimGroup                                                                                */
        asm!("{} {}", out(reg) _, inlateout(reg) b => _);                                                                                 /*
        asm!("{}•{}",•out(reg)•_,•inlateout(reg)•b•=>•_);    ExpressionStatement
        asm!("{}•{}",•out(reg)•_,•inlateout(reg)•b•=>•_)     MacroInvocation
             "{}•{}"                                         Literal
                    ,                                        PunctuationToken
                         (reg)                               DelimGroup
                               _                             PunctuationToken
                                ,                            PunctuationToken
                                           (reg)             DelimGroup
                                                   =>        PunctuationToken
                                                      _      PunctuationToken                                                             */
        asm!("", out("al") _, lateout("rcx") _);                                                                                          /*
        asm!("",•out("al")•_,•lateout("rcx")•_);    ExpressionStatement
        asm!("",•out("al")•_,•lateout("rcx")•_)     MacroInvocation
             ""                                     Literal
               ,                                    PunctuationToken
                    ("al")                          DelimGroup
                     "al"                           Literal
                           _                        PunctuationToken
                            ,                       PunctuationToken
                                     ("rcx")        DelimGroup
                                      "rcx"         Literal
                                             _      PunctuationToken                                                                      */
        asm!("beep~", "boop!");                                                                                                           /*
        asm!("beep~",•"boop!");    ExpressionStatement
        asm!("beep~",•"boop!")     MacroInvocation
             "beep~"               Literal
                    ,              PunctuationToken
                      "boop!"      Literal                                                                                                */
        asm!("beep~ {}, 42", "boop! {}, 24", in(reg) a, out(reg) b);                                                                      /*
        asm!("beep~•{},•42",•"boop!•{},•24",•in(reg)•a,•out(reg)•b);    ExpressionStatement
        asm!("beep~•{},•42",•"boop!•{},•24",•in(reg)•a,•out(reg)•b)     MacroInvocation
             "beep~•{},•42"                                             Literal
                           ,                                            PunctuationToken
                             "boop!•{},•24"                             Literal
                                           ,                            PunctuationToken
                                               (reg)                    DelimGroup
                                                      ,                 PunctuationToken
                                                           (reg)        DelimGroup                                                        */
        asm!("boop! {1}, 24", "beep~ {0}, 42", in(reg) a, out(reg) b);                                                                    /*
        asm!("boop!•{1},•24",•"beep~•{0},•42",•in(reg)•a,•out(reg)•b);    ExpressionStatement
        asm!("boop!•{1},•24",•"beep~•{0},•42",•in(reg)•a,•out(reg)•b)     MacroInvocation
             "boop!•{1},•24"                                              Literal
                            ,                                             PunctuationToken
                              "beep~•{0},•42"                             Literal
                                             ,                            PunctuationToken
                                                 (reg)                    DelimGroup
                                                        ,                 PunctuationToken
                                                             (reg)        DelimGroup                                                      */
        asm!("beep~ {}, 42", "boop! {name}, 24", in(reg) a, name = out(reg) b);                                                           /*
        asm!("beep~•{},•42",•"boop!•{name},•24",•in(reg)•a,•name•=•out(reg)•b);    ExpressionStatement
        asm!("beep~•{},•42",•"boop!•{name},•24",•in(reg)•a,•name•=•out(reg)•b)     MacroInvocation
             "beep~•{},•42"                                                        Literal
                           ,                                                       PunctuationToken
                             "boop!•{name},•24"                                    Literal
                                               ,                                   PunctuationToken
                                                   (reg)                           DelimGroup
                                                          ,                        PunctuationToken
                                                                 =                 PunctuationToken
                                                                      (reg)        DelimGroup                                             */
        asm!(                                                                                                                             /*
        asm!(↲    <ExpressionStatement>, <MacroInvocation>                                                                                */
            "beep~                                                                                                                       "/*
            "beep~↲    <Literal>                                                                                                         */"
boop!"                                                                                                                                    /*
boop!"    </Literal>                                                                                                                      */
        );                                                                                                                                /*
••••••••);    </ExpressionStatement>
••••••••)     </MacroInvocation>                                                                                                          */
        asm!("beep~\nboop!");                                                                                                             /*
        asm!("beep~\nboop!");    ExpressionStatement
        asm!("beep~\nboop!")     MacroInvocation
             "beep~\nboop!"      Literal                                                                                                  */
        asm!("beep~\n\tboop!");                                                                                                           /*
        asm!("beep~\n\tboop!");    ExpressionStatement
        asm!("beep~\n\tboop!")     MacroInvocation
             "beep~\n\tboop!"      Literal                                                                                                */
        asm!("beep~\nboop!", "boop3\nboop4");                                                                                             /*
        asm!("beep~\nboop!",•"boop3\nboop4");    ExpressionStatement
        asm!("beep~\nboop!",•"boop3\nboop4")     MacroInvocation
             "beep~\nboop!"                      Literal
                           ,                     PunctuationToken
                             "boop3\nboop4"      Literal                                                                                  */
    }                                                                                                                                     /*
••••}    </ExpressionStatement>, </BlockExpression>                                                                                       */
}                                                                                                                                         /*
}    </FunctionDeclaration>                                                                                                               */
debug!(?value);                                                                                                                           /*
debug!(?value);    ExpressionStatement
debug!(?value)     MacroInvocation
       ?           PunctuationToken                                                                                                       */
debug!(                                                                                                                                   /*
debug!(↲    <ExpressionStatement>, <MacroInvocation>                                                                                      */
    "VariantDef::new(name = {:?}, variant_did = {:?}, ctor_def_id = {:?}, discr = {:?},                                                  "/*
    "VariantDef::new(name•=•{:?},•variant_did•=•{:?},•ctor_def_id•=•{:?},•discr•=•{:?},↲    <Literal>                                    */"
     fields = {:?}, ctor_kind = {:?}, adt_kind = {:?}, parent_did = {:?})",                                                               /*
•••••fields•=•{:?},•ctor_kind•=•{:?},•adt_kind•=•{:?},•parent_did•=•{:?})"     </Literal>
                                                                          ,    PunctuationToken                                           */
    name, variant_did, ctor_def_id, discr, fields, ctor_kind, adt_kind, parent_did,                                                       /*
        ,                                                                              PunctuationToken
                     ,                                                                 PunctuationToken
                                  ,                                                    PunctuationToken
                                         ,                                             PunctuationToken
                                                 ,                                     PunctuationToken
                                                            ,                          PunctuationToken
                                                                      ,                PunctuationToken
                                                                                  ,    PunctuationToken                                   */
);                                                                                                                                        /*
);    </ExpressionStatement>
)     </MacroInvocation>                                                                                                                  */
slice_interners!(                                                                                                                         /*
slice_interners!(↲    <ExpressionStatement>, <MacroInvocation>                                                                            */
    substs: _intern_substs(GenericArg<'tcx>),                                                                                             /*
          :                                      PunctuationToken
                          (GenericArg<'tcx>)     DelimGroup
                                     <           PunctuationToken
                                      'tcx       LtIdentifier
                                          >      PunctuationToken
                                            ,    PunctuationToken                                                                         */
    canonical_var_infos: _intern_canonical_var_infos(CanonicalVarInfo<'tcx>),                                                             /*
                       :                                                         PunctuationToken
                                                    (CanonicalVarInfo<'tcx>)     DelimGroup
                                                                     <           PunctuationToken
                                                                      'tcx       LtIdentifier
                                                                          >      PunctuationToken
                                                                            ,    PunctuationToken                                         */
    poly_existential_predicates:                                                                                                          /*
                               :    PunctuationToken                                                                                      */
        _intern_poly_existential_predicates(ty::Binder<'tcx, ExistentialPredicate<'tcx>>),                                                /*
                                           (ty::Binder<'tcx,•ExistentialPredicate<'tcx>>)     DelimGroup
                                              ::                                              PunctuationToken
                                                      <                                       PunctuationToken
                                                       'tcx                                   LtIdentifier
                                                           ,                                  PunctuationToken
                                                                                 <            PunctuationToken
                                                                                  'tcx        LtIdentifier
                                                                                      >>      PunctuationToken
                                                                                         ,    PunctuationToken                            */
    predicates: _intern_predicates(Predicate<'tcx>),                                                                                      /*
              :                                         PunctuationToken
                                  (Predicate<'tcx>)     DelimGroup
                                            <           PunctuationToken
                                             'tcx       LtIdentifier
                                                 >      PunctuationToken
                                                   ,    PunctuationToken                                                                  */
    projs: _intern_projs(ProjectionKind),                                                                                                 /*
         :                                   PunctuationToken
                        (ProjectionKind)     DelimGroup
                                        ,    PunctuationToken                                                                             */
    place_elems: _intern_place_elems(PlaceElem<'tcx>),                                                                                    /*
               :                                          PunctuationToken
                                    (PlaceElem<'tcx>)     DelimGroup
                                              <           PunctuationToken
                                               'tcx       LtIdentifier
                                                   >      PunctuationToken
                                                     ,    PunctuationToken                                                                */
    bound_variable_kinds: _intern_bound_variable_kinds(ty::BoundVariableKind),                                                            /*
                        :                                                         PunctuationToken
                                                      (ty::BoundVariableKind)     DelimGroup
                                                         ::                       PunctuationToken
                                                                             ,    PunctuationToken                                        */
);                                                                                                                                        /*
);    </ExpressionStatement>
)     </MacroInvocation>                                                                                                                  */

impl_binder_encode_decode! {                                                                                                              /*
impl_binder_encode_decode!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                 */
    &'tcx ty::List<Ty<'tcx>>,                                                                                                             /*
    &                            PunctuationToken
     'tcx                        LtIdentifier
            ::                   PunctuationToken
                  <              PunctuationToken
                     <           PunctuationToken
                      'tcx       LtIdentifier
                          >>     PunctuationToken
                            ,    PunctuationToken                                                                                         */
    ty::FnSig<'tcx>,                                                                                                                      /*
      ::                PunctuationToken
             <          PunctuationToken
              'tcx      LtIdentifier
                  >     PunctuationToken
                   ,    PunctuationToken                                                                                                  */
    ty::ExistentialPredicate<'tcx>,                                                                                                       /*
      ::                               PunctuationToken
                            <          PunctuationToken
                             'tcx      LtIdentifier
                                 >     PunctuationToken
                                  ,    PunctuationToken                                                                                   */
    ty::TraitRef<'tcx>,                                                                                                                   /*
      ::                   PunctuationToken
                <          PunctuationToken
                 'tcx      LtIdentifier
                     >     PunctuationToken
                      ,    PunctuationToken                                                                                               */
    Vec<ty::GeneratorInteriorTypeCause<'tcx>>,                                                                                            /*
       <                                          PunctuationToken
          ::                                      PunctuationToken
                                      <           PunctuationToken
                                       'tcx       LtIdentifier
                                           >>     PunctuationToken
                                             ,    PunctuationToken                                                                        */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */
impl_arena_copy_decoder! {<'tcx>                                                                                                          /*
impl_arena_copy_decoder!•{<'tcx>↲    <ExpressionStatement>, <MacroInvocation>
                          <          PunctuationToken
                           'tcx      LtIdentifier
                               >     PunctuationToken                                                                                     */
    Span,                                                                                                                                 /*
        ,    PunctuationToken                                                                                                             */
    rustc_span::symbol::Ident,                                                                                                            /*
              ::                  PunctuationToken
                      ::          PunctuationToken
                             ,    PunctuationToken                                                                                        */
    ty::Variance,                                                                                                                         /*
      ::             PunctuationToken
                ,    PunctuationToken                                                                                                     */
    rustc_span::def_id::DefId,                                                                                                            /*
              ::                  PunctuationToken
                      ::          PunctuationToken
                             ,    PunctuationToken                                                                                        */
    rustc_span::def_id::LocalDefId,                                                                                                       /*
              ::                       PunctuationToken
                      ::               PunctuationToken
                                  ,    PunctuationToken                                                                                   */
    (rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>, rustc_middle::middle::exported_symbols::SymbolExportInfo),             /*
    (rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>,•rustc_middle::middle::exported_symbols::SymbolExportInfo)     DelimGroup
                 ::                                                                                                              PunctuationToken
                         ::                                                                                                      PunctuationToken
                                           ::                                                                                    PunctuationToken
                                                           <                                                                     PunctuationToken
                                                            'tcx                                                                 LtIdentifier
                                                                >                                                                PunctuationToken
                                                                 ,                                                               PunctuationToken
                                                                               ::                                                PunctuationToken
                                                                                       ::                                        PunctuationToken
                                                                                                         ::                      PunctuationToken
                                                                                                                            ,    PunctuationToken*/
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */

bitflags! {                                                                                                                               /*
bitflags!•{↲    <ExpressionStatement>, <MacroInvocation>                                                                                  */
    #[derive(HashStable, TyEncodable, TyDecodable)]                                                                                       /*
    #                                                  PunctuationToken
     [derive(HashStable,•TyEncodable,•TyDecodable)]    DelimGroup
            (HashStable,•TyEncodable,•TyDecodable)     DelimGroup
                       ,                               PunctuationToken
                                    ,                  PunctuationToken                                                                   */
    pub struct AdtFlags: u32 {                                                                                                            /*
                       :           PunctuationToken
                             {↲    <DelimGroup>                                                                                           */
        const NO_ADT_FLAGS        = 0;                                                                                                    /*
                                  =       PunctuationToken
                                    0     Literal
                                     ;    PunctuationToken                                                                                */
        /// Indicates whether the ADT is an enum.
        ///•Indicates•whether•the•ADT•is•an•enum.    Comment
        const IS_ENUM             = 1 << 0;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         0     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the ADT is a union.
        ///•Indicates•whether•the•ADT•is•a•union.    Comment
        const IS_UNION            = 1 << 1;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         1     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the ADT is a struct.
        ///•Indicates•whether•the•ADT•is•a•struct.    Comment
        const IS_STRUCT           = 1 << 2;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         2     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the ADT is a struct and has a constructor.
        ///•Indicates•whether•the•ADT•is•a•struct•and•has•a•constructor.    Comment
        const HAS_CTOR            = 1 << 3;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         3     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the type is `PhantomData`.
        ///•Indicates•whether•the•type•is•`PhantomData`.    Comment
        const IS_PHANTOM_DATA     = 1 << 4;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         4     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the type has a `#[fundamental]` attribute.
        ///•Indicates•whether•the•type•has•a•`#[fundamental]`•attribute.    Comment
        const IS_FUNDAMENTAL      = 1 << 5;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         5     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the type is `Box`.
        ///•Indicates•whether•the•type•is•`Box`.    Comment
        const IS_BOX              = 1 << 6;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         6     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the type is `ManuallyDrop`.
        ///•Indicates•whether•the•type•is•`ManuallyDrop`.    Comment
        const IS_MANUALLY_DROP    = 1 << 7;                                                                                               /*
                                  =            PunctuationToken
                                    1          Literal
                                      <<       PunctuationToken
                                         7     Literal
                                          ;    PunctuationToken                                                                           */
        /// Indicates whether the variant list of this ADT is `#[non_exhaustive]`.
        ///•Indicates•whether•the•variant•list•of•this•ADT•is•`#[non_exhaustive]`.    Comment
        /// (i.e., this flag is never set unless this ADT is an enum).
        ///•(i.e.,•this•flag•is•never•set•unless•this•ADT•is•an•enum).    Comment
        const IS_VARIANT_LIST_NON_EXHAUSTIVE = 1 << 8;                                                                                    /*
                                             =            PunctuationToken
                                               1          Literal
                                                 <<       PunctuationToken
                                                    8     Literal
                                                     ;    PunctuationToken                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
}                                                                                                                                         /*
}    </ExpressionStatement>, </MacroInvocation>                                                                                           */
rustc_dep_node_append!([define_dep_nodes!][ <'tcx>                                                                                        /*
rustc_dep_node_append!([define_dep_nodes!][•<'tcx>↲    <ExpressionStatement>, <MacroInvocation>
                       [define_dep_nodes!]             DelimGroup
                                        !              PunctuationToken
                                          [•<'tcx>↲    <DelimGroup>
                                            <          PunctuationToken
                                             'tcx      LtIdentifier
                                                 >     PunctuationToken                                                                   */
    // We use this for most things when incr. comp. is turned off.
    //•We•use•this•for•most•things•when•incr.•comp.•is•turned•off.    Comment
    [] Null,                                                                                                                              /*
    []          DelimGroup
           ,    PunctuationToken                                                                                                          */

    [anon] TraitSelect,                                                                                                                   /*
    [anon]                 DelimGroup
                      ,    PunctuationToken                                                                                               */

    // WARNING: if `Symbol` is changed, make sure you update `make_compile_codegen_unit` below.
    //•WARNING:•if•`Symbol`•is•changed,•make•sure•you•update•`make_compile_codegen_unit`•below.    Comment
    [] CompileCodegenUnit(Symbol),                                                                                                        /*
    []                                DelimGroup
                         (Symbol)     DelimGroup
                                 ,    PunctuationToken                                                                                    */

    // WARNING: if `MonoItem` is changed, make sure you update `make_compile_mono_item` below.
    //•WARNING:•if•`MonoItem`•is•changed,•make•sure•you•update•`make_compile_mono_item`•below.    Comment
    // Only used by rustc_codegen_cranelift
    //•Only•used•by•rustc_codegen_cranelift    Comment
    [] CompileMonoItem(MonoItem),                                                                                                         /*
    []                               DelimGroup
                      (MonoItem)     DelimGroup
                                ,    PunctuationToken                                                                                     */
]);                                                                                                                                       /*
]);    </ExpressionStatement>
])     </MacroInvocation>
]      </DelimGroup>                                                                                                                      */

decl_derive!([Decodable] => serialize::decodable_derive);                                                                                 /*
decl_derive!([Decodable]•=>•serialize::decodable_derive);    ExpressionStatement
decl_derive!([Decodable]•=>•serialize::decodable_derive)     MacroInvocation
             [Decodable]                                     DelimGroup
                         =>                                  PunctuationToken
                                     ::                      PunctuationToken                                                             */

let ret = structure.gen_impl(quote! {                                                                                                     /*
let•ret•=•structure.gen_impl(quote!•{↲    <LetVariableDeclaration>
          structure.gen_impl(quote!•{↲    <CallExpression>
                             quote!•{↲    <MacroInvocation>                                                                               */
    gen impl rustc_errors::AddSubdiagnostic for @Self {                                                                                   /*
                         ::                                 PunctuationToken
                                                @           PunctuationToken
                                                      {↲    <DelimGroup>                                                                  */
        fn add_to_diagnostic(self, #diag: &mut rustc_errors::Diagnostic) {                                                                /*
                            (self,•#diag:•&mut•rustc_errors::Diagnostic)       DelimGroup
                                 ,                                             PunctuationToken
                                   #                                           PunctuationToken
                                        :                                      PunctuationToken
                                          &                                    PunctuationToken
                                                           ::                  PunctuationToken
                                                                         {↲    <DelimGroup>                                               */
            use rustc_errors::{Applicability, IntoDiagnosticArg};                                                                         /*
                            ::                                       PunctuationToken
                              {Applicability,•IntoDiagnosticArg}     DelimGroup
                                            ,                        PunctuationToken
                                                                ;    PunctuationToken                                                     */
            #implementation                                                                                                               /*
            #                  PunctuationToken                                                                                           */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */
    }                                                                                                                                     /*
••••}    </DelimGroup>                                                                                                                    */
});                                                                                                                                       /*
});    </LetVariableDeclaration>
})     </CallExpression>
}      </MacroInvocation>                                                                                                                 */

// Discarded Nodes: 0
// Parsed Nodes: 2190
// state_rollbacks: 2
// Total '.charCodeAt()' calls: 12299 (13% re-reads)
// Unnecessary 'skip_whitespace()' calls: 712
// source: "../../samples/macro/macro.tokens.rs"