
pub macro c {                                                                                                                             /*
pub•macro•c•{↲    <Program>
pub•macro•c•{↲    <Program.ast{dk: "None"}>
pub•macro•c•{↲    <MacroDeclaration>
pub               PubSpecifier
            {↲    <MacroDeclaration.rules{dk: "{}"}>                                                                                      */
    () => (),                                                                                                                             /*
    ()•=>•()    MacroRuleDeclaration
    ()          MacroRuleDeclaration.match{dk: "()"}
          ()    MacroRuleDeclaration.transform{dk: "()"}                                                                                  */
    // _______
    //•_______    Comment{line}
    // _______
    //•_______    Comment{line}
    ($msg:expr $(,)?) => (),                                                                                                              /*
    ($msg:expr•$(,)?)•=>•()    MacroRuleDeclaration
    ($msg:expr•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     $msg:expr                 MacroParameterDeclaration
     $msg                      McIdentifier
               $(,)?           MacroGroup{kind: "?"}
                (,)            MacroGroup.segments{dk: "()"}
                 ,             PunctuationToken{tk: ","}
                         ()    MacroRuleDeclaration.transform{dk: "()"}                                                                   */
    ($fmt:expr, $($arg:tt)*) => (),                                                                                                       /*
    ($fmt:expr,•$($arg:tt)*)•=>•()    MacroRuleDeclaration
    ($fmt:expr,•$($arg:tt)*)          MacroRuleDeclaration.match{dk: "()"}
     $fmt:expr                        MacroParameterDeclaration
     $fmt                             McIdentifier
              ,                       PunctuationToken{tk: ","}
                $($arg:tt)*           MacroGroup{kind: "*"}
                 ($arg:tt)            MacroGroup.segments{dk: "()"}
                  $arg:tt             MacroParameterDeclaration
                  $arg                McIdentifier
                                ()    MacroRuleDeclaration.transform{dk: "()"}                                                            */
    ($foo:expr) => {};                                                                                                                    /*
    ($foo:expr)•=>•{}    MacroRuleDeclaration
    ($foo:expr)          MacroRuleDeclaration.match{dk: "()"}
     $foo:expr           MacroParameterDeclaration
     $foo                McIdentifier
                   {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                         */
    ($($t:tt)+) => (),                                                                                                                    /*
    ($($t:tt)+)•=>•()    MacroRuleDeclaration
    ($($t:tt)+)          MacroRuleDeclaration.match{dk: "()"}
     $($t:tt)+           MacroGroup{kind: "+"}
      ($t:tt)            MacroGroup.segments{dk: "()"}
       $t:tt             MacroParameterDeclaration
       $t                McIdentifier
                   ()    MacroRuleDeclaration.transform{dk: "()"}                                                                         */
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => (),                                                      /*
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?•$(,)?)•=>•()    MacroRuleDeclaration
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     $left:expr                                                                        MacroParameterDeclaration
     $left                                                                             McIdentifier
               ,                                                                       PunctuationToken{tk: ","}
                 $(|)?                                                                 MacroGroup{kind: "?"}
                  (|)                                                                  MacroGroup.segments{dk: "()"}
                   |                                                                   PunctuationToken{tk: "|"}
                       $(•$pattern:pat_param•)|+                                       MacroGroup{kind: "+"}
                        (•$pattern:pat_param•)                                         MacroGroup.segments{dk: "()"}
                          $pattern:pat_param                                           MacroParameterDeclaration
                          $pattern                                                     McIdentifier
                                              |                                        PunctuationToken{tk: "|"}
                                                 $(•if•$guard:•expr•)?                 MacroGroup{kind: "?"}
                                                  (•if•$guard:•expr•)                  MacroGroup.segments{dk: "()"}
                                                       $guard:•expr                    MacroParameterDeclaration
                                                       $guard                          McIdentifier
                                                                       $(,)?           MacroGroup{kind: "?"}
                                                                        (,)            MacroGroup.segments{dk: "()"}
                                                                         ,             PunctuationToken{tk: ","}
                                                                                 ()    MacroRuleDeclaration.transform{dk: "()"}           */
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $($arg:tt)+) => (),                                               /*
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?,•$($arg:tt)+)•=>•()    MacroRuleDeclaration
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?,•$($arg:tt)+)          MacroRuleDeclaration.match{dk: "()"}
     $left:expr                                                                               MacroParameterDeclaration
     $left                                                                                    McIdentifier
               ,                                                                              PunctuationToken{tk: ","}
                 $(|)?                                                                        MacroGroup{kind: "?"}
                  (|)                                                                         MacroGroup.segments{dk: "()"}
                   |                                                                          PunctuationToken{tk: "|"}
                       $(•$pattern:pat_param•)|+                                              MacroGroup{kind: "+"}
                        (•$pattern:pat_param•)                                                MacroGroup.segments{dk: "()"}
                          $pattern:pat_param                                                  MacroParameterDeclaration
                          $pattern                                                            McIdentifier
                                              |                                               PunctuationToken{tk: "|"}
                                                 $(•if•$guard:•expr•)?                        MacroGroup{kind: "?"}
                                                  (•if•$guard:•expr•)                         MacroGroup.segments{dk: "()"}
                                                       $guard:•expr                           MacroParameterDeclaration
                                                       $guard                                 McIdentifier
                                                                      ,                       PunctuationToken{tk: ","}
                                                                        $($arg:tt)+           MacroGroup{kind: "+"}
                                                                         ($arg:tt)            MacroGroup.segments{dk: "()"}
                                                                          $arg:tt             MacroParameterDeclaration
                                                                          $arg                McIdentifier
                                                                                        ()    MacroRuleDeclaration.transform{dk: "()"}    */
    // ______
    //•______    Comment{line}
    ("{}", $aze:expr $(,)?) => (),                                                                                                        /*
    ("{}",•$aze:expr•$(,)?)•=>•()    MacroRuleDeclaration
    ("{}",•$aze:expr•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     "{}"                            Literal{kind: String}
         ,                           PunctuationToken{tk: ","}
           $aze:expr                 MacroParameterDeclaration
           $aze                      McIdentifier
                     $(,)?           MacroGroup{kind: "?"}
                      (,)            MacroGroup.segments{dk: "()"}
                       ,             PunctuationToken{tk: ","}
                               ()    MacroRuleDeclaration.transform{dk: "()"}                                                             */
    ($($t:tt)+) => (),                                                                                                                    /*
    ($($t:tt)+)•=>•()    MacroRuleDeclaration
    ($($t:tt)+)          MacroRuleDeclaration.match{dk: "()"}
     $($t:tt)+           MacroGroup{kind: "+"}
      ($t:tt)            MacroGroup.segments{dk: "()"}
       $t:tt             MacroParameterDeclaration
       $t                McIdentifier
                   ()    MacroRuleDeclaration.transform{dk: "()"}                                                                         */
    ($x:pat | $y:pat) => {}                                                                                                               /*
    ($x:pat•|•$y:pat)•=>•{}    MacroRuleDeclaration
    ($x:pat•|•$y:pat)          MacroRuleDeclaration.match{dk: "()"}
     $x:pat                    MacroParameterDeclaration
     $x                        McIdentifier
            |                  PunctuationToken{tk: "|"}
              $y:pat           MacroParameterDeclaration
              $y               McIdentifier
                         {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                   */
    ($($x:pat)+ | $($y:pat)+) => {}                                                                                                       /*
    ($($x:pat)+•|•$($y:pat)+)•=>•{}    MacroRuleDeclaration
    ($($x:pat)+•|•$($y:pat)+)          MacroRuleDeclaration.match{dk: "()"}
     $($x:pat)+                        MacroGroup{kind: "+"}
      ($x:pat)                         MacroGroup.segments{dk: "()"}
       $x:pat                          MacroParameterDeclaration
       $x                              McIdentifier
                |                      PunctuationToken{tk: "|"}
                  $($y:pat)+           MacroGroup{kind: "+"}
                   ($y:pat)            MacroGroup.segments{dk: "()"}
                    $y:pat             MacroParameterDeclaration
                    $y                 McIdentifier
                                 {}    MacroRuleDeclaration.transform{dk: "{}"}                                                           */
    ($x:pat_param | $y:pat_param) => {}                                                                                                   /*
    ($x:pat_param•|•$y:pat_param)•=>•{}    MacroRuleDeclaration
    ($x:pat_param•|•$y:pat_param)          MacroRuleDeclaration.match{dk: "()"}
     $x:pat_param                          MacroParameterDeclaration
     $x                                    McIdentifier
                  |                        PunctuationToken{tk: "|"}
                    $y:pat_param           MacroParameterDeclaration
                    $y                     McIdentifier
                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                       */
    ($x:pat_param | $y:pat) => {}                                                                                                         /*
    ($x:pat_param•|•$y:pat)•=>•{}    MacroRuleDeclaration
    ($x:pat_param•|•$y:pat)          MacroRuleDeclaration.match{dk: "()"}
     $x:pat_param                    MacroParameterDeclaration
     $x                              McIdentifier
                  |                  PunctuationToken{tk: "|"}
                    $y:pat           MacroParameterDeclaration
                    $y               McIdentifier
                               {}    MacroRuleDeclaration.transform{dk: "{}"}                                                             */
    ($x:pat | $y:pat_param) => {}                                                                                                         /*
    ($x:pat•|•$y:pat_param)•=>•{}    MacroRuleDeclaration
    ($x:pat•|•$y:pat_param)          MacroRuleDeclaration.match{dk: "()"}
     $x:pat                          MacroParameterDeclaration
     $x                              McIdentifier
            |                        PunctuationToken{tk: "|"}
              $y:pat_param           MacroParameterDeclaration
              $y                     McIdentifier
                               {}    MacroRuleDeclaration.transform{dk: "{}"}                                                             */
}                                                                                                                                         /*
}    </MacroDeclaration.rules>
}    </MacroDeclaration>                                                                                                                  */

macro_rules! c {                                                                                                                          /*
macro_rules!•c•{↲    <MacroRulesDeclaration>
               {↲    <MacroRulesDeclaration.rules{dk: "{}"}>                                                                              */
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}
    //
    //    Comment{line}
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}
    //
    //    Comment{line}
    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}

    // ____
    //•____    Comment{line}
    (@ {                                                                                                                                  /*
    (@•{↲    <MacroRuleDeclaration>
    (@•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     @       PunctuationToken{tk: "@"}
       {↲    <DelimGroup>                                                                                                                 */
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        start=$start:expr;                                                                                                                /*
             =                PunctuationToken{tk: "="}
              $start:expr     MacroParameterDeclaration
              $start          McIdentifier
                         ;    PunctuationToken{tk: ";"}                                                                                   */

        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        ( $($count:tt)* )                                                                                                                 /*
        (•$($count:tt)*•)    DelimGroup
          $($count:tt)*      MacroGroup{kind: "*"}
           ($count:tt)       MacroGroup.segments{dk: "()"}
            $count:tt        MacroParameterDeclaration
            $count           McIdentifier                                                                                                 */

        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        $( ( $($skip:tt)* ) $bind:pat = $fut:expr, if $c:expr => $handle:expr, )+                                                         /*
        $(•(•$($skip:tt)*•)•$bind:pat•=•$fut:expr,•if•$c:expr•=>•$handle:expr,•)+    MacroGroup{kind: "+"}
         (•(•$($skip:tt)*•)•$bind:pat•=•$fut:expr,•if•$c:expr•=>•$handle:expr,•)     MacroGroup.segments{dk: "()"}
           (•$($skip:tt)*•)                                                          DelimGroup
             $($skip:tt)*                                                            MacroGroup{kind: "*"}
              ($skip:tt)                                                             MacroGroup.segments{dk: "()"}
               $skip:tt                                                              MacroParameterDeclaration
               $skip                                                                 McIdentifier
                            $bind:pat                                                MacroParameterDeclaration
                            $bind                                                    McIdentifier
                                      =                                              PunctuationToken{tk: "="}
                                        $fut:expr                                    MacroParameterDeclaration
                                        $fut                                         McIdentifier
                                                 ,                                   PunctuationToken{tk: ","}
                                                      $c:expr                        MacroParameterDeclaration
                                                      $c                             McIdentifier
                                                              =>                     PunctuationToken{tk: "=>"}
                                                                 $handle:expr        MacroParameterDeclaration
                                                                 $handle             McIdentifier
                                                                             ,       PunctuationToken{tk: ","}                            */

        // ____
        //•____    Comment{line}
        ; $else:expr                                                                                                                      /*
        ;               PunctuationToken{tk: ";"}
          $else:expr    MacroParameterDeclaration
          $else         McIdentifier                                                                                                      */

    }) => {};                                                                                                                             /*
••••}           </DelimGroup>
••••})          </MacroRuleDeclaration.match>
          {}    MacroRuleDeclaration.transform{dk: "{}"}
••••})•=>•{}    </MacroRuleDeclaration>                                                                                                   */

    // ____
    //•____    Comment{line}

    // ____
    //•____    Comment{line}
    // ____
    //•____    Comment{line}

    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr, if $c:expr => $h:block, $($r:tt)* ) => {};                        /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:block,•$($r:tt)*•)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:block,•$($r:tt)*•)          MacroRuleDeclaration.match{dk: "()"}
     @                                                                                                               PunctuationToken{tk: "@"}
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                                DelimGroup
              =                                                                                                      PunctuationToken{tk: "="}
               $start:expr                                                                                           MacroParameterDeclaration
               $start                                                                                                McIdentifier
                          ;                                                                                          PunctuationToken{tk: ";"}
                            (•$($s:tt)*•)                                                                            DelimGroup
                              $($s:tt)*                                                                              MacroGroup{kind: "*"}
                               ($s:tt)                                                                               MacroGroup.segments{dk: "()"}
                                $s:tt                                                                                MacroParameterDeclaration
                                $s                                                                                   McIdentifier
                                          $($t:tt)*                                                                  MacroGroup{kind: "*"}
                                           ($t:tt)                                                                   MacroGroup.segments{dk: "()"}
                                            $t:tt                                                                    MacroParameterDeclaration
                                            $t                                                                       McIdentifier
                                                      $p:pat                                                         MacroParameterDeclaration
                                                      $p                                                             McIdentifier
                                                             =                                                       PunctuationToken{tk: "="}
                                                               $f:expr                                               MacroParameterDeclaration
                                                               $f                                                    McIdentifier
                                                                      ,                                              PunctuationToken{tk: ","}
                                                                           $c:expr                                   MacroParameterDeclaration
                                                                           $c                                        McIdentifier
                                                                                   =>                                PunctuationToken{tk: "=>"}
                                                                                      $h:block                       MacroParameterDeclaration
                                                                                      $h                             McIdentifier
                                                                                              ,                      PunctuationToken{tk: ","}
                                                                                                $($r:tt)*            MacroGroup{kind: "*"}
                                                                                                 ($r:tt)             MacroGroup.segments{dk: "()"}
                                                                                                  $r:tt              MacroParameterDeclaration
                                                                                                  $r                 McIdentifier
                                                                                                               {}    MacroRuleDeclaration.transform{dk: "{}"}*/
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr, if $c:expr => $h:expr ) => {};                                    /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:expr•)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:expr•)          MacroRuleDeclaration.match{dk: "()"}
     @                                                                                                   PunctuationToken{tk: "@"}
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                    DelimGroup
              =                                                                                          PunctuationToken{tk: "="}
               $start:expr                                                                               MacroParameterDeclaration
               $start                                                                                    McIdentifier
                          ;                                                                              PunctuationToken{tk: ";"}
                            (•$($s:tt)*•)                                                                DelimGroup
                              $($s:tt)*                                                                  MacroGroup{kind: "*"}
                               ($s:tt)                                                                   MacroGroup.segments{dk: "()"}
                                $s:tt                                                                    MacroParameterDeclaration
                                $s                                                                       McIdentifier
                                          $($t:tt)*                                                      MacroGroup{kind: "*"}
                                           ($t:tt)                                                       MacroGroup.segments{dk: "()"}
                                            $t:tt                                                        MacroParameterDeclaration
                                            $t                                                           McIdentifier
                                                      $p:pat                                             MacroParameterDeclaration
                                                      $p                                                 McIdentifier
                                                             =                                           PunctuationToken{tk: "="}
                                                               $f:expr                                   MacroParameterDeclaration
                                                               $f                                        McIdentifier
                                                                      ,                                  PunctuationToken{tk: ","}
                                                                           $c:expr                       MacroParameterDeclaration
                                                                           $c                            McIdentifier
                                                                                   =>                    PunctuationToken{tk: "=>"}
                                                                                      $h:expr            MacroParameterDeclaration
                                                                                      $h                 McIdentifier
                                                                                                   {}    MacroRuleDeclaration.transform{dk: "{}"}*/
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr => $h:expr, $($r:tt)* ) => {};                                     /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr,•$($r:tt)*•)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr,•$($r:tt)*•)          MacroRuleDeclaration.match{dk: "()"}
     @                                                                                                  PunctuationToken{tk: "@"}
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                   DelimGroup
              =                                                                                         PunctuationToken{tk: "="}
               $start:expr                                                                              MacroParameterDeclaration
               $start                                                                                   McIdentifier
                          ;                                                                             PunctuationToken{tk: ";"}
                            (•$($s:tt)*•)                                                               DelimGroup
                              $($s:tt)*                                                                 MacroGroup{kind: "*"}
                               ($s:tt)                                                                  MacroGroup.segments{dk: "()"}
                                $s:tt                                                                   MacroParameterDeclaration
                                $s                                                                      McIdentifier
                                          $($t:tt)*                                                     MacroGroup{kind: "*"}
                                           ($t:tt)                                                      MacroGroup.segments{dk: "()"}
                                            $t:tt                                                       MacroParameterDeclaration
                                            $t                                                          McIdentifier
                                                      $p:pat                                            MacroParameterDeclaration
                                                      $p                                                McIdentifier
                                                             =                                          PunctuationToken{tk: "="}
                                                               $f:expr                                  MacroParameterDeclaration
                                                               $f                                       McIdentifier
                                                                       =>                               PunctuationToken{tk: "=>"}
                                                                          $h:expr                       MacroParameterDeclaration
                                                                          $h                            McIdentifier
                                                                                 ,                      PunctuationToken{tk: ","}
                                                                                   $($r:tt)*            MacroGroup{kind: "*"}
                                                                                    ($r:tt)             MacroGroup.segments{dk: "()"}
                                                                                     $r:tt              MacroParameterDeclaration
                                                                                     $r                 McIdentifier
                                                                                                  {}    MacroRuleDeclaration.transform{dk: "{}"}*/
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr => $h:expr ) => {};                                                /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr•)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr•)          MacroRuleDeclaration.match{dk: "()"}
     @                                                                                       PunctuationToken{tk: "@"}
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                        DelimGroup
              =                                                                              PunctuationToken{tk: "="}
               $start:expr                                                                   MacroParameterDeclaration
               $start                                                                        McIdentifier
                          ;                                                                  PunctuationToken{tk: ";"}
                            (•$($s:tt)*•)                                                    DelimGroup
                              $($s:tt)*                                                      MacroGroup{kind: "*"}
                               ($s:tt)                                                       MacroGroup.segments{dk: "()"}
                                $s:tt                                                        MacroParameterDeclaration
                                $s                                                           McIdentifier
                                          $($t:tt)*                                          MacroGroup{kind: "*"}
                                           ($t:tt)                                           MacroGroup.segments{dk: "()"}
                                            $t:tt                                            MacroParameterDeclaration
                                            $t                                               McIdentifier
                                                      $p:pat                                 MacroParameterDeclaration
                                                      $p                                     McIdentifier
                                                             =                               PunctuationToken{tk: "="}
                                                               $f:expr                       MacroParameterDeclaration
                                                               $f                            McIdentifier
                                                                       =>                    PunctuationToken{tk: "=>"}
                                                                          $h:expr            MacroParameterDeclaration
                                                                          $h                 McIdentifier
                                                                                       {}    MacroRuleDeclaration.transform{dk: "{}"}     */
    (@ { start=$start:expr; $($t:tt)* } else => $else:expr $(,)?) => {};                                                                  /*
    (@•{•start=$start:expr;•$($t:tt)*•}•else•=>•$else:expr•$(,)?)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•$($t:tt)*•}•else•=>•$else:expr•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     @                                                                     PunctuationToken{tk: "@"}
       {•start=$start:expr;•$($t:tt)*•}                                    DelimGroup
              =                                                            PunctuationToken{tk: "="}
               $start:expr                                                 MacroParameterDeclaration
               $start                                                      McIdentifier
                          ;                                                PunctuationToken{tk: ";"}
                            $($t:tt)*                                      MacroGroup{kind: "*"}
                             ($t:tt)                                       MacroGroup.segments{dk: "()"}
                              $t:tt                                        MacroParameterDeclaration
                              $t                                           McIdentifier
                                             =>                            PunctuationToken{tk: "=>"}
                                                $else:expr                 MacroParameterDeclaration
                                                $else                      McIdentifier
                                                           $(,)?           MacroGroup{kind: "?"}
                                                            (,)            MacroGroup.segments{dk: "()"}
                                                             ,             PunctuationToken{tk: ","}
                                                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                       */
    (@ { start=$start:expr; $($t:tt)* } ) => {};                                                                                          /*
    (@•{•start=$start:expr;•$($t:tt)*•}•)•=>•{}    MacroRuleDeclaration
    (@•{•start=$start:expr;•$($t:tt)*•}•)          MacroRuleDeclaration.match{dk: "()"}
     @                                             PunctuationToken{tk: "@"}
       {•start=$start:expr;•$($t:tt)*•}            DelimGroup
              =                                    PunctuationToken{tk: "="}
               $start:expr                         MacroParameterDeclaration
               $start                              McIdentifier
                          ;                        PunctuationToken{tk: ";"}
                            $($t:tt)*              MacroGroup{kind: "*"}
                             ($t:tt)               MacroGroup.segments{dk: "()"}
                              $t:tt                MacroParameterDeclaration
                              $t                   McIdentifier
                                             {}    MacroRuleDeclaration.transform{dk: "{}"}                                               */

    ( ($($id:ident),*) ) => (());                                                                                                         /*
    (•($($id:ident),*)•)•=>•(())    MacroRuleDeclaration
    (•($($id:ident),*)•)            MacroRuleDeclaration.match{dk: "()"}
      ($($id:ident),*)              DelimGroup
       $($id:ident),*               MacroGroup{kind: "*"}
        ($id:ident)                 MacroGroup.segments{dk: "()"}
         $id:ident                  MacroParameterDeclaration
         $id                        McIdentifier
                   ,                PunctuationToken{tk: ","}
                            (())    MacroRuleDeclaration.transform{dk: "()"}
                             ()     DelimGroup                                                                                            */
    ( [$($id:ident),*] ) => (());                                                                                                         /*
    (•[$($id:ident),*]•)•=>•(())    MacroRuleDeclaration
    (•[$($id:ident),*]•)            MacroRuleDeclaration.match{dk: "()"}
      [$($id:ident),*]              DelimGroup
       $($id:ident),*               MacroGroup{kind: "*"}
        ($id:ident)                 MacroGroup.segments{dk: "()"}
         $id:ident                  MacroParameterDeclaration
         $id                        McIdentifier
                   ,                PunctuationToken{tk: ","}
                            (())    MacroRuleDeclaration.transform{dk: "()"}
                             ()     DelimGroup                                                                                            */
    ( {$($id:ident),*} ) => (());                                                                                                         /*
    (•{$($id:ident),*}•)•=>•(())    MacroRuleDeclaration
    (•{$($id:ident),*}•)            MacroRuleDeclaration.match{dk: "()"}
      {$($id:ident),*}              DelimGroup
       $($id:ident),*               MacroGroup{kind: "*"}
        ($id:ident)                 MacroGroup.segments{dk: "()"}
         $id:ident                  MacroParameterDeclaration
         $id                        McIdentifier
                   ,                PunctuationToken{tk: ","}
                            (())    MacroRuleDeclaration.transform{dk: "()"}
                             ()     DelimGroup                                                                                            */

    // ____
    //•____    Comment{line}

    (biased; $p:pat = $($t:tt)* ) => {};                                                                                                  /*
    (biased;•$p:pat•=•$($t:tt)*•)•=>•{}    MacroRuleDeclaration
    (biased;•$p:pat•=•$($t:tt)*•)          MacroRuleDeclaration.match{dk: "()"}
           ;                               PunctuationToken{tk: ";"}
             $p:pat                        MacroParameterDeclaration
             $p                            McIdentifier
                    =                      PunctuationToken{tk: "="}
                      $($t:tt)*            MacroGroup{kind: "*"}
                       ($t:tt)             MacroGroup.segments{dk: "()"}
                        $t:tt              MacroParameterDeclaration
                        $t                 McIdentifier
                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                                       */

    ( $p:pat = $($t:tt)* ) => {};                                                                                                         /*
    (•$p:pat•=•$($t:tt)*•)•=>•{}    MacroRuleDeclaration
    (•$p:pat•=•$($t:tt)*•)          MacroRuleDeclaration.match{dk: "()"}
      $p:pat                        MacroParameterDeclaration
      $p                            McIdentifier
             =                      PunctuationToken{tk: "="}
               $($t:tt)*            MacroGroup{kind: "*"}
                ($t:tt)             MacroGroup.segments{dk: "()"}
                 $t:tt              MacroParameterDeclaration
                 $t                 McIdentifier
                              {}    MacroRuleDeclaration.transform{dk: "{}"}                                                              */
    () => {};                                                                                                                             /*
    ()•=>•{}    MacroRuleDeclaration
    ()          MacroRuleDeclaration.match{dk: "()"}
          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                                  */
    ($($tts:tt)+) => { loom::thread_local!{ $($tts)+ } };                                                                                 /*
    ($($tts:tt)+)•=>•{•loom::thread_local!{•$($tts)+•}•}    MacroRuleDeclaration
    ($($tts:tt)+)                                           MacroRuleDeclaration.match{dk: "()"}
     $($tts:tt)+                                            MacroGroup{kind: "+"}
      ($tts:tt)                                             MacroGroup.segments{dk: "()"}
       $tts:tt                                              MacroParameterDeclaration
       $tts                                                 McIdentifier
                     {•loom::thread_local!{•$($tts)+•}•}    MacroRuleDeclaration.transform{dk: "{}"}
                           ::                               PunctuationToken{tk: "::"}
                                         !                  PunctuationToken{tk: "!"}
                                          {•$($tts)+•}      DelimGroup
                                            $($tts)+        MacroGroup{kind: "+"}
                                             ($tts)         MacroGroup.segments{dk: "()"}
                                              $tts          McIdentifier                                                                  */
    (@ {                                                                                                                                  /*
    (@•{↲    <MacroRuleDeclaration>
    (@•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     @       PunctuationToken{tk: "@"}
       {↲    <DelimGroup>                                                                                                                 */
        // ____
        //•____    Comment{line}
        // ____
        //•____    Comment{line}
        ( $($count:tt)* )                                                                                                                 /*
        (•$($count:tt)*•)    DelimGroup
          $($count:tt)*      MacroGroup{kind: "*"}
           ($count:tt)       MacroGroup.segments{dk: "()"}
            $count:tt        MacroParameterDeclaration
            $count           McIdentifier                                                                                                 */

        // ____
        //•____    Comment{line}
        $( ( $($skip:tt)* ) $e:expr, )*                                                                                                   /*
        $(•(•$($skip:tt)*•)•$e:expr,•)*    MacroGroup{kind: "*"}
         (•(•$($skip:tt)*•)•$e:expr,•)     MacroGroup.segments{dk: "()"}
           (•$($skip:tt)*•)                DelimGroup
             $($skip:tt)*                  MacroGroup{kind: "*"}
              ($skip:tt)                   MacroGroup.segments{dk: "()"}
               $skip:tt                    MacroParameterDeclaration
               $skip                       McIdentifier
                            $e:expr        MacroParameterDeclaration
                            $e             McIdentifier
                                   ,       PunctuationToken{tk: ","}                                                                      */

    }) => {};                                                                                                                             /*
••••}           </DelimGroup>
••••})          </MacroRuleDeclaration.match>
          {}    MacroRuleDeclaration.transform{dk: "{}"}
••••})•=>•{}    </MacroRuleDeclaration>                                                                                                   */

    // ____
    //•____    Comment{line}

    (@ { ( $($s:tt)* ) $($t:tt)* } $e:expr, $($r:tt)* ) => {};                                                                            /*
    (@•{•(•$($s:tt)*•)•$($t:tt)*•}•$e:expr,•$($r:tt)*•)•=>•{}    MacroRuleDeclaration
    (@•{•(•$($s:tt)*•)•$($t:tt)*•}•$e:expr,•$($r:tt)*•)          MacroRuleDeclaration.match{dk: "()"}
     @                                                           PunctuationToken{tk: "@"}
       {•(•$($s:tt)*•)•$($t:tt)*•}                               DelimGroup
         (•$($s:tt)*•)                                           DelimGroup
           $($s:tt)*                                             MacroGroup{kind: "*"}
            ($s:tt)                                              MacroGroup.segments{dk: "()"}
             $s:tt                                               MacroParameterDeclaration
             $s                                                  McIdentifier
                       $($t:tt)*                                 MacroGroup{kind: "*"}
                        ($t:tt)                                  MacroGroup.segments{dk: "()"}
                         $t:tt                                   MacroParameterDeclaration
                         $t                                      McIdentifier
                                   $e:expr                       MacroParameterDeclaration
                                   $e                            McIdentifier
                                          ,                      PunctuationToken{tk: ","}
                                            $($r:tt)*            MacroGroup{kind: "*"}
                                             ($r:tt)             MacroGroup.segments{dk: "()"}
                                              $r:tt              MacroParameterDeclaration
                                              $r                 McIdentifier
                                                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                 */
    ($($(#[$cfg:meta])* $name:ident,)*) => {};                                                                                            /*
    ($($(#[$cfg:meta])*•$name:ident,)*)•=>•{}    MacroRuleDeclaration
    ($($(#[$cfg:meta])*•$name:ident,)*)          MacroRuleDeclaration.match{dk: "()"}
     $($(#[$cfg:meta])*•$name:ident,)*           MacroGroup{kind: "*"}
      ($(#[$cfg:meta])*•$name:ident,)            MacroGroup.segments{dk: "()"}
       $(#[$cfg:meta])*                          MacroGroup{kind: "*"}
        (#[$cfg:meta])                           MacroGroup.segments{dk: "()"}
         #                                       PunctuationToken{tk: "#"}
          [$cfg:meta]                            DelimGroup
           $cfg:meta                             MacroParameterDeclaration
           $cfg                                  McIdentifier
                        $name:ident              MacroParameterDeclaration
                        $name                    McIdentifier
                                   ,             PunctuationToken{tk: ","}
                                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                                 */

    // ____
    //•____    Comment{line}

    ( $($e:expr),* $(,)?) => {};                                                                                                          /*
    (•$($e:expr),*•$(,)?)•=>•{}    MacroRuleDeclaration
    (•$($e:expr),*•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
      $($e:expr),*                 MacroGroup{kind: "*"}
       ($e:expr)                   MacroGroup.segments{dk: "()"}
        $e:expr                    MacroParameterDeclaration
        $e                         McIdentifier
                ,                  PunctuationToken{tk: ","}
                   $(,)?           MacroGroup{kind: "?"}
                    (,)            MacroGroup.segments{dk: "()"}
                     ,             PunctuationToken{tk: ","}
                             {}    MacroRuleDeclaration.transform{dk: "{}"}                                                               */
    () => {};                                                                                                                             /*
    ()•=>•{}    MacroRuleDeclaration
    ()          MacroRuleDeclaration.match{dk: "()"}
          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                                  */
    
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => {};                                                           /*
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty;•$($rest:tt)*)•=>•{}    MacroRuleDeclaration
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty;•$($rest:tt)*)          MacroRuleDeclaration.match{dk: "()"}
     $(#[$attr:meta])*                                                            MacroGroup{kind: "*"}
      (#[$attr:meta])                                                             MacroGroup.segments{dk: "()"}
       #                                                                          PunctuationToken{tk: "#"}
        [$attr:meta]                                                              DelimGroup
         $attr:meta                                                               MacroParameterDeclaration
         $attr                                                                    McIdentifier
                       $vis:vis                                                   MacroParameterDeclaration
                       $vis                                                       McIdentifier
                                       $name:ident                                MacroParameterDeclaration
                                       $name                                      McIdentifier
                                                  :                               PunctuationToken{tk: ":"}
                                                    $t:ty                         MacroParameterDeclaration
                                                    $t                            McIdentifier
                                                         ;                        PunctuationToken{tk: ";"}
                                                           $($rest:tt)*           MacroGroup{kind: "*"}
                                                            ($rest:tt)            MacroGroup.segments{dk: "()"}
                                                             $rest:tt             MacroParameterDeclaration
                                                             $rest                McIdentifier
                                                                            {}    MacroRuleDeclaration.transform{dk: "{}"}                */
    
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty) => {}                                                                          /*
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty)•=>•{}    MacroRuleDeclaration
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty)          MacroRuleDeclaration.match{dk: "()"}
     $(#[$attr:meta])*                                              MacroGroup{kind: "*"}
      (#[$attr:meta])                                               MacroGroup.segments{dk: "()"}
       #                                                            PunctuationToken{tk: "#"}
        [$attr:meta]                                                DelimGroup
         $attr:meta                                                 MacroParameterDeclaration
         $attr                                                      McIdentifier
                       $vis:vis                                     MacroParameterDeclaration
                       $vis                                         McIdentifier
                                       $name:ident                  MacroParameterDeclaration
                                       $name                        McIdentifier
                                                  :                 PunctuationToken{tk: ":"}
                                                    $t:ty           MacroParameterDeclaration
                                                    $t              McIdentifier
                                                              {}    MacroRuleDeclaration.transform{dk: "{}"}                              */
    (Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};                                                                                   /*
    (Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)•=>•{}    MacroRuleDeclaration
    (Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)          MacroRuleDeclaration.match{dk: "()"}
          &                                               PunctuationToken{tk: "&"}
            $(!)?                                         MacroGroup{kind: "?"}
             (!)                                          MacroGroup.segments{dk: "()"}
              !                                           PunctuationToken{tk: "!"}
                      &                                   PunctuationToken{tk: "&"}
                        $(!)?                             MacroGroup{kind: "?"}
                         (!)                              MacroGroup.segments{dk: "()"}
                          !                               PunctuationToken{tk: "!"}
                                  ,                       PunctuationToken{tk: ","}
                                    $value:expr           MacroParameterDeclaration
                                    $value                McIdentifier
                                                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                        */
    (!Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};                                                                                  /*
    (!Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)•=>•{}    MacroRuleDeclaration
    (!Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)          MacroRuleDeclaration.match{dk: "()"}
     !                                                     PunctuationToken{tk: "!"}
           &                                               PunctuationToken{tk: "&"}
             $(!)?                                         MacroGroup{kind: "?"}
              (!)                                          MacroGroup.segments{dk: "()"}
               !                                           PunctuationToken{tk: "!"}
                       &                                   PunctuationToken{tk: "&"}
                         $(!)?                             MacroGroup{kind: "?"}
                          (!)                              MacroGroup.segments{dk: "()"}
                           !                               PunctuationToken{tk: "!"}
                                   ,                       PunctuationToken{tk: ","}
                                     $value:expr           MacroParameterDeclaration
                                     $value                McIdentifier
                                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                       */
    ($($f:ident $(< $($generic:ty),* > )? )::+($($arg:ty),*): $($tok:tt)*) => {};                                                         /*
    ($($f:ident•$(<•$($generic:ty),*•>•)?•)::+($($arg:ty),*):•$($tok:tt)*)•=>•{}    MacroRuleDeclaration
    ($($f:ident•$(<•$($generic:ty),*•>•)?•)::+($($arg:ty),*):•$($tok:tt)*)          MacroRuleDeclaration.match{dk: "()"}
     $($f:ident•$(<•$($generic:ty),*•>•)?•)::+                                      MacroGroup{kind: "+"}
      ($f:ident•$(<•$($generic:ty),*•>•)?•)                                         MacroGroup.segments{dk: "()"}
       $f:ident                                                                     MacroParameterDeclaration
       $f                                                                           McIdentifier
                $(<•$($generic:ty),*•>•)?                                           MacroGroup{kind: "?"}
                 (<•$($generic:ty),*•>•)                                            MacroGroup.segments{dk: "()"}
                  <                                                                 PunctuationToken{tk: "<"}
                    $($generic:ty),*                                                MacroGroup{kind: "*"}
                     ($generic:ty)                                                  MacroGroup.segments{dk: "()"}
                      $generic:ty                                                   MacroParameterDeclaration
                      $generic                                                      McIdentifier
                                  ,                                                 PunctuationToken{tk: ","}
                                     >                                              PunctuationToken{tk: ">"}
                                           ::                                       PunctuationToken{tk: "::"}
                                              ($($arg:ty),*)                        DelimGroup
                                               $($arg:ty),*                         MacroGroup{kind: "*"}
                                                ($arg:ty)                           MacroGroup.segments{dk: "()"}
                                                 $arg:ty                            MacroParameterDeclaration
                                                 $arg                               McIdentifier
                                                         ,                          PunctuationToken{tk: ","}
                                                            :                       PunctuationToken{tk: ":"}
                                                              $($tok:tt)*           MacroGroup{kind: "*"}
                                                               ($tok:tt)            MacroGroup.segments{dk: "()"}
                                                                $tok:tt             MacroParameterDeclaration
                                                                $tok                McIdentifier
                                                                              {}    MacroRuleDeclaration.transform{dk: "{}"}              */
    ($i:ident, $start:ident, $($delta:expr),*$(,)?) => {};                                                                                /*
    ($i:ident,•$start:ident,•$($delta:expr),*$(,)?)•=>•{}    MacroRuleDeclaration
    ($i:ident,•$start:ident,•$($delta:expr),*$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                                                MacroParameterDeclaration
     $i                                                      McIdentifier
             ,                                               PunctuationToken{tk: ","}
               $start:ident                                  MacroParameterDeclaration
               $start                                        McIdentifier
                           ,                                 PunctuationToken{tk: ","}
                             $($delta:expr),*                MacroGroup{kind: "*"}
                              ($delta:expr)                  MacroGroup.segments{dk: "()"}
                               $delta:expr                   MacroParameterDeclaration
                               $delta                        McIdentifier
                                           ,                 PunctuationToken{tk: ","}
                                             $(,)?           MacroGroup{kind: "?"}
                                              (,)            MacroGroup.segments{dk: "()"}
                                               ,             PunctuationToken{tk: ","}
                                                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                     */
    ($i:ident, $start:ident) => {};                                                                                                       /*
    ($i:ident,•$start:ident)•=>•{}    MacroRuleDeclaration
    ($i:ident,•$start:ident)          MacroRuleDeclaration.match{dk: "()"}
     $i:ident                         MacroParameterDeclaration
     $i                               McIdentifier
             ,                        PunctuationToken{tk: ","}
               $start:ident           MacroParameterDeclaration
               $start                 McIdentifier
                                {}    MacroRuleDeclaration.transform{dk: "{}"}                                                            */
    ($($name:ident = $sem:ident($bits:tt : $exp_bits:tt)),*) => {};                                                                       /*
    ($($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt)),*)•=>•{}    MacroRuleDeclaration
    ($($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt)),*)          MacroRuleDeclaration.match{dk: "()"}
     $($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt)),*           MacroGroup{kind: "*"}
      ($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt))             MacroGroup.segments{dk: "()"}
       $name:ident                                                    MacroParameterDeclaration
       $name                                                          McIdentifier
                   =                                                  PunctuationToken{tk: "="}
                     $sem:ident                                       MacroParameterDeclaration
                     $sem                                             McIdentifier
                               ($bits:tt•:•$exp_bits:tt)              DelimGroup
                                $bits:tt                              MacroParameterDeclaration
                                $bits                                 McIdentifier
                                         :                            PunctuationToken{tk: ":"}
                                           $exp_bits:tt               MacroParameterDeclaration
                                           $exp_bits                  McIdentifier
                                                         ,            PunctuationToken{tk: ","}
                                                                {}    MacroRuleDeclaration.transform{dk: "{}"}                            */
    ($ty:ident<$t:tt>) => {};                                                                                                             /*
    ($ty:ident<$t:tt>)•=>•{}    MacroRuleDeclaration
    ($ty:ident<$t:tt>)          MacroRuleDeclaration.match{dk: "()"}
     $ty:ident                  MacroParameterDeclaration
     $ty                        McIdentifier
              <                 PunctuationToken{tk: "<"}
               $t:tt            MacroParameterDeclaration
               $t               McIdentifier
                    >           PunctuationToken{tk: ">"}
                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                  */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        const SUPPORTS_CUSTOM_INNER_ATTRS: bool = $inner_attrs:literal;                                                                   /*
                                         :                                 PunctuationToken{tk: ":"}
                                                =                          PunctuationToken{tk: "="}
                                                  $inner_attrs:literal     MacroParameterDeclaration
                                                  $inner_attrs             McIdentifier
                                                                      ;    PunctuationToken{tk: ";"}                                      */
        $($ty:path),*                                                                                                                     /*
        $($ty:path),*    MacroGroup{kind: "*"}
         ($ty:path)      MacroGroup.segments{dk: "()"}
          $ty:path       MacroParameterDeclaration
          $ty            McIdentifier
                   ,     PunctuationToken{tk: ","}                                                                                        */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    ($($name:literal => $feature:ident)*) => {};                                                                                          /*
    ($($name:literal•=>•$feature:ident)*)•=>•{}    MacroRuleDeclaration
    ($($name:literal•=>•$feature:ident)*)          MacroRuleDeclaration.match{dk: "()"}
     $($name:literal•=>•$feature:ident)*           MacroGroup{kind: "*"}
      ($name:literal•=>•$feature:ident)            MacroGroup.segments{dk: "()"}
       $name:literal                               MacroParameterDeclaration
       $name                                       McIdentifier
                     =>                            PunctuationToken{tk: "=>"}
                        $feature:ident             MacroParameterDeclaration
                        $feature                   McIdentifier
                                             {}    MacroRuleDeclaration.transform{dk: "{}"}                                               */
    ( $(                                                                                                                                  /*
    (•$(↲    <MacroRuleDeclaration>
    (•$(↲    <MacroRuleDeclaration.match{dk: "()"}>
      $(↲    <MacroGroup{kind: "*"}>
       (↲    <MacroGroup.segments{dk: "()"}>                                                                                              */
        $T:ident { $( $field:ident : $A:ident ),* $(,)? }                                                                                 /*
        $T:ident                                             MacroParameterDeclaration
        $T                                                   McIdentifier
                 {•$(•$field:ident•:•$A:ident•),*•$(,)?•}    DelimGroup
                   $(•$field:ident•:•$A:ident•),*            MacroGroup{kind: "*"}
                    (•$field:ident•:•$A:ident•)              MacroGroup.segments{dk: "()"}
                      $field:ident                           MacroParameterDeclaration
                      $field                                 McIdentifier
                                   :                         PunctuationToken{tk: ":"}
                                     $A:ident                MacroParameterDeclaration
                                     $A                      McIdentifier
                                               ,             PunctuationToken{tk: ","}
                                                  $(,)?      MacroGroup{kind: "?"}
                                                   (,)       MacroGroup.segments{dk: "()"}
                                                    ,        PunctuationToken{tk: ","}                                                    */
    )* ) => {};                                                                                                                           /*
••••)             </MacroGroup.segments>
••••)*            </MacroGroup>
••••)*•)          </MacroRuleDeclaration.match>
            {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)*•)•=>•{}    </MacroRuleDeclaration>                                                                                                 */
    ($wr:ident . write_facts_to_path($this:ident . [                                                                                      /*
    ($wr:ident•.•write_facts_to_path($this:ident•.•[↲    <MacroRuleDeclaration>
    ($wr:ident•.•write_facts_to_path($this:ident•.•[↲    <MacroRuleDeclaration.match{dk: "()"}>
     $wr:ident                                           MacroParameterDeclaration
     $wr                                                 McIdentifier
               .                                         PunctuationToken{tk: "."}
                                    ($this:ident•.•[↲    <DelimGroup>
                                     $this:ident         MacroParameterDeclaration
                                     $this               McIdentifier
                                                 .       PunctuationToken{tk: "."}
                                                   [↲    <DelimGroup>                                                                     */
        $($field:ident,)*                                                                                                                 /*
        $($field:ident,)*    MacroGroup{kind: "*"}
         ($field:ident,)     MacroGroup.segments{dk: "()"}
          $field:ident       MacroParameterDeclaration
          $field             McIdentifier
                      ,      PunctuationToken{tk: ","}                                                                                    */
    ])) => {};                                                                                                                            /*
••••]            </DelimGroup>
••••])           </DelimGroup>
••••]))          </MacroRuleDeclaration.match>
           {}    MacroRuleDeclaration.transform{dk: "{}"}
••••]))•=>•{}    </MacroRuleDeclaration>                                                                                                  */
    ($in_:expr, {                                                                                                                         /*
    ($in_:expr,•{↲    <MacroRuleDeclaration>
    ($in_:expr,•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     $in_:expr        MacroParameterDeclaration
     $in_             McIdentifier
              ,       PunctuationToken{tk: ","}
                {↲    <DelimGroup>                                                                                                        */
        $param:expr, $flags:expr,                                                                                                         /*
        $param:expr                  MacroParameterDeclaration
        $param                       McIdentifier
                   ,                 PunctuationToken{tk: ","}
                     $flags:expr     MacroParameterDeclaration
                     $flags          McIdentifier
                                ,    PunctuationToken{tk: ","}                                                                            */
        $width:expr, $prec:expr, $len:expr, $type_:expr,                                                                                  /*
        $width:expr                                         MacroParameterDeclaration
        $width                                              McIdentifier
                   ,                                        PunctuationToken{tk: ","}
                     $prec:expr                             MacroParameterDeclaration
                     $prec                                  McIdentifier
                               ,                            PunctuationToken{tk: ","}
                                 $len:expr                  MacroParameterDeclaration
                                 $len                       McIdentifier
                                          ,                 PunctuationToken{tk: ","}
                                            $type_:expr     MacroParameterDeclaration
                                            $type_          McIdentifier
                                                       ,    PunctuationToken{tk: ","}                                                     */
        $pos:expr,                                                                                                                        /*
        $pos:expr     MacroParameterDeclaration
        $pos          McIdentifier
                 ,    PunctuationToken{tk: ","}                                                                                           */
    }) => {};                                                                                                                             /*
••••}           </DelimGroup>
••••})          </MacroRuleDeclaration.match>
          {}    MacroRuleDeclaration.transform{dk: "{}"}
••••})•=>•{}    </MacroRuleDeclaration>                                                                                                   */
    (($($dollar:tt $placeholder:ident)*); $($($values:ident),+);*: $($test:tt)*) => {};                                                   /*
    (($($dollar:tt•$placeholder:ident)*);•$($($values:ident),+);*:•$($test:tt)*)•=>•{}    MacroRuleDeclaration
    (($($dollar:tt•$placeholder:ident)*);•$($($values:ident),+);*:•$($test:tt)*)          MacroRuleDeclaration.match{dk: "()"}
     ($($dollar:tt•$placeholder:ident)*)                                                  DelimGroup
      $($dollar:tt•$placeholder:ident)*                                                   MacroGroup{kind: "*"}
       ($dollar:tt•$placeholder:ident)                                                    MacroGroup.segments{dk: "()"}
        $dollar:tt                                                                        MacroParameterDeclaration
        $dollar                                                                           McIdentifier
                   $placeholder:ident                                                     MacroParameterDeclaration
                   $placeholder                                                           McIdentifier
                                        ;                                                 PunctuationToken{tk: ";"}
                                          $($($values:ident),+);*                         MacroGroup{kind: "*"}
                                           ($($values:ident),+)                           MacroGroup.segments{dk: "()"}
                                            $($values:ident),+                            MacroGroup{kind: "+"}
                                             ($values:ident)                              MacroGroup.segments{dk: "()"}
                                              $values:ident                               MacroParameterDeclaration
                                              $values                                     McIdentifier
                                                            ,                             PunctuationToken{tk: ","}
                                                               ;                          PunctuationToken{tk: ";"}
                                                                 :                        PunctuationToken{tk: ":"}
                                                                   $($test:tt)*           MacroGroup{kind: "*"}
                                                                    ($test:tt)            MacroGroup.segments{dk: "()"}
                                                                     $test:tt             MacroParameterDeclaration
                                                                     $test                McIdentifier
                                                                                    {}    MacroRuleDeclaration.transform{dk: "{}"}        */
    ($($name: ident: $($($p: ident),* => $call: ident),*;)*) => {};                                                                       /*
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)•=>•{}    MacroRuleDeclaration
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)          MacroRuleDeclaration.match{dk: "()"}
     $($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*           MacroGroup{kind: "*"}
      ($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)            MacroGroup.segments{dk: "()"}
       $name:•ident                                                   MacroParameterDeclaration
       $name                                                          McIdentifier
                   :                                                  PunctuationToken{tk: ":"}
                     $($($p:•ident),*•=>•$call:•ident),*              MacroGroup{kind: "*"}
                      ($($p:•ident),*•=>•$call:•ident)                MacroGroup.segments{dk: "()"}
                       $($p:•ident),*                                 MacroGroup{kind: "*"}
                        ($p:•ident)                                   MacroGroup.segments{dk: "()"}
                         $p:•ident                                    MacroParameterDeclaration
                         $p                                           McIdentifier
                                   ,                                  PunctuationToken{tk: ","}
                                      =>                              PunctuationToken{tk: "=>"}
                                         $call:•ident                 MacroParameterDeclaration
                                         $call                        McIdentifier
                                                      ,               PunctuationToken{tk: ","}
                                                        ;             PunctuationToken{tk: ";"}
                                                                {}    MacroRuleDeclaration.transform{dk: "{}"}                            */
    ($($name:ident($($arg:ident),*) => $llvm_capi:ident),+ $(,)?) => {};                                                                  /*
    ($($name:ident($($arg:ident),*)•=>•$llvm_capi:ident),+•$(,)?)•=>•{}    MacroRuleDeclaration
    ($($name:ident($($arg:ident),*)•=>•$llvm_capi:ident),+•$(,)?)          MacroRuleDeclaration.match{dk: "()"}
     $($name:ident($($arg:ident),*)•=>•$llvm_capi:ident),+                 MacroGroup{kind: "+"}
      ($name:ident($($arg:ident),*)•=>•$llvm_capi:ident)                   MacroGroup.segments{dk: "()"}
       $name:ident                                                         MacroParameterDeclaration
       $name                                                               McIdentifier
                  ($($arg:ident),*)                                        DelimGroup
                   $($arg:ident),*                                         MacroGroup{kind: "*"}
                    ($arg:ident)                                           MacroGroup.segments{dk: "()"}
                     $arg:ident                                            MacroParameterDeclaration
                     $arg                                                  McIdentifier
                                ,                                          PunctuationToken{tk: ","}
                                    =>                                     PunctuationToken{tk: "=>"}
                                       $llvm_capi:ident                    MacroParameterDeclaration
                                       $llvm_capi                          McIdentifier
                                                        ,                  PunctuationToken{tk: ","}
                                                           $(,)?           MacroGroup{kind: "?"}
                                                            (,)            MacroGroup.segments{dk: "()"}
                                                             ,             PunctuationToken{tk: ","}
                                                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                       */
    ($name:expr, fn() -> $ret:expr) => {};                                                                                                /*
    ($name:expr,•fn()•->•$ret:expr)•=>•{}    MacroRuleDeclaration
    ($name:expr,•fn()•->•$ret:expr)          MacroRuleDeclaration.match{dk: "()"}
     $name:expr                              MacroParameterDeclaration
     $name                                   McIdentifier
               ,                             PunctuationToken{tk: ","}
                   ()                        DelimGroup
                      ->                     PunctuationToken{tk: "->"}
                         $ret:expr           MacroParameterDeclaration
                         $ret                McIdentifier
                                       {}    MacroRuleDeclaration.transform{dk: "{}"}                                                     */
    ($name:expr, fn(...) -> $ret:expr) => {};                                                                                             /*
    ($name:expr,•fn(...)•->•$ret:expr)•=>•{}    MacroRuleDeclaration
    ($name:expr,•fn(...)•->•$ret:expr)          MacroRuleDeclaration.match{dk: "()"}
     $name:expr                                 MacroParameterDeclaration
     $name                                      McIdentifier
               ,                                PunctuationToken{tk: ","}
                   (...)                        DelimGroup
                    ...                         PunctuationToken{tk: "..."}
                         ->                     PunctuationToken{tk: "->"}
                            $ret:expr           MacroParameterDeclaration
                            $ret                McIdentifier
                                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                                  */
    ($name:expr, fn($($arg:expr),*) -> $ret:expr) => {};                                                                                  /*
    ($name:expr,•fn($($arg:expr),*)•->•$ret:expr)•=>•{}    MacroRuleDeclaration
    ($name:expr,•fn($($arg:expr),*)•->•$ret:expr)          MacroRuleDeclaration.match{dk: "()"}
     $name:expr                                            MacroParameterDeclaration
     $name                                                 McIdentifier
               ,                                           PunctuationToken{tk: ","}
                   ($($arg:expr),*)                        DelimGroup
                    $($arg:expr),*                         MacroGroup{kind: "*"}
                     ($arg:expr)                           MacroGroup.segments{dk: "()"}
                      $arg:expr                            MacroParameterDeclaration
                      $arg                                 McIdentifier
                                ,                          PunctuationToken{tk: ","}
                                    ->                     PunctuationToken{tk: "->"}
                                       $ret:expr           MacroParameterDeclaration
                                       $ret                McIdentifier
                                                     {}    MacroRuleDeclaration.transform{dk: "{}"}                                       */
    ($($field_ty:expr),*) => {};                                                                                                          /*
    ($($field_ty:expr),*)•=>•{}    MacroRuleDeclaration
    ($($field_ty:expr),*)          MacroRuleDeclaration.match{dk: "()"}
     $($field_ty:expr),*           MacroGroup{kind: "*"}
      ($field_ty:expr)             MacroGroup.segments{dk: "()"}
       $field_ty:expr              MacroParameterDeclaration
       $field_ty                   McIdentifier
                      ,            PunctuationToken{tk: ","}
                             {}    MacroRuleDeclaration.transform{dk: "{}"}                                                               */
    ($($name: ident: $($($p: ident),* => $call: ident),*;)*) => {};                                                                       /*
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)•=>•{}    MacroRuleDeclaration
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)          MacroRuleDeclaration.match{dk: "()"}
     $($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*           MacroGroup{kind: "*"}
      ($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)            MacroGroup.segments{dk: "()"}
       $name:•ident                                                   MacroParameterDeclaration
       $name                                                          McIdentifier
                   :                                                  PunctuationToken{tk: ":"}
                     $($($p:•ident),*•=>•$call:•ident),*              MacroGroup{kind: "*"}
                      ($($p:•ident),*•=>•$call:•ident)                MacroGroup.segments{dk: "()"}
                       $($p:•ident),*                                 MacroGroup{kind: "*"}
                        ($p:•ident)                                   MacroGroup.segments{dk: "()"}
                         $p:•ident                                    MacroParameterDeclaration
                         $p                                           McIdentifier
                                   ,                                  PunctuationToken{tk: ","}
                                      =>                              PunctuationToken{tk: "=>"}
                                         $call:•ident                 MacroParameterDeclaration
                                         $call                        McIdentifier
                                                      ,               PunctuationToken{tk: ","}
                                                        ;             PunctuationToken{tk: ";"}
                                                                {}    MacroRuleDeclaration.transform{dk: "{}"}                            */
    ($where:expr, { $( $what_fmt:expr ),+ } $( expected { $( $expected_fmt:expr ),+ } )?) => {};                                          /*
    ($where:expr,•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?)•=>•{}    MacroRuleDeclaration
    ($where:expr,•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?)          MacroRuleDeclaration.match{dk: "()"}
     $where:expr                                                                                   MacroParameterDeclaration
     $where                                                                                        McIdentifier
                ,                                                                                  PunctuationToken{tk: ","}
                  {•$(•$what_fmt:expr•),+•}                                                        DelimGroup
                    $(•$what_fmt:expr•),+                                                          MacroGroup{kind: "+"}
                     (•$what_fmt:expr•)                                                            MacroGroup.segments{dk: "()"}
                       $what_fmt:expr                                                              MacroParameterDeclaration
                       $what_fmt                                                                   McIdentifier
                                       ,                                                           PunctuationToken{tk: ","}
                                            $(•expected•{•$(•$expected_fmt:expr•),+•}•)?           MacroGroup{kind: "?"}
                                             (•expected•{•$(•$expected_fmt:expr•),+•}•)            MacroGroup.segments{dk: "()"}
                                                        {•$(•$expected_fmt:expr•),+•}              DelimGroup
                                                          $(•$expected_fmt:expr•),+                MacroGroup{kind: "+"}
                                                           (•$expected_fmt:expr•)                  MacroGroup.segments{dk: "()"}
                                                             $expected_fmt:expr                    MacroParameterDeclaration
                                                             $expected_fmt                         McIdentifier
                                                                                 ,                 PunctuationToken{tk: ","}
                                                                                             {}    MacroRuleDeclaration.transform{dk: "{}"}*/
    ($e:expr, $where:expr,                                                                                                                /*
    ($e:expr,•$where:expr,↲    <MacroRuleDeclaration>
    ($e:expr,•$where:expr,↲    <MacroRuleDeclaration.match{dk: "()"}>
     $e:expr                   MacroParameterDeclaration
     $e                        McIdentifier
            ,                  PunctuationToken{tk: ","}
              $where:expr      MacroParameterDeclaration
              $where           McIdentifier
                         ,     PunctuationToken{tk: ","}                                                                                  */
    $( $( $p:pat_param )|+ => { $( $what_fmt:expr ),+ } $( expected { $( $expected_fmt:expr ),+ } )? ),+ $(,)?                            /*
    $(•$(•$p:pat_param•)|+•=>•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?•),+          MacroGroup{kind: "+"}
     (•$(•$p:pat_param•)|+•=>•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?•)            MacroGroup.segments{dk: "()"}
       $(•$p:pat_param•)|+                                                                                        MacroGroup{kind: "+"}
        (•$p:pat_param•)                                                                                          MacroGroup.segments{dk: "()"}
          $p:pat_param                                                                                            MacroParameterDeclaration
          $p                                                                                                      McIdentifier
                        |                                                                                         PunctuationToken{tk: "|"}
                           =>                                                                                     PunctuationToken{tk: "=>"}
                              {•$(•$what_fmt:expr•),+•}                                                           DelimGroup
                                $(•$what_fmt:expr•),+                                                             MacroGroup{kind: "+"}
                                 (•$what_fmt:expr•)                                                               MacroGroup.segments{dk: "()"}
                                   $what_fmt:expr                                                                 MacroParameterDeclaration
                                   $what_fmt                                                                      McIdentifier
                                                   ,                                                              PunctuationToken{tk: ","}
                                                        $(•expected•{•$(•$expected_fmt:expr•),+•}•)?              MacroGroup{kind: "?"}
                                                         (•expected•{•$(•$expected_fmt:expr•),+•}•)               MacroGroup.segments{dk: "()"}
                                                                    {•$(•$expected_fmt:expr•),+•}                 DelimGroup
                                                                      $(•$expected_fmt:expr•),+                   MacroGroup{kind: "+"}
                                                                       (•$expected_fmt:expr•)                     MacroGroup.segments{dk: "()"}
                                                                         $expected_fmt:expr                       MacroParameterDeclaration
                                                                         $expected_fmt                            McIdentifier
                                                                                             ,                    PunctuationToken{tk: ","}
                                                                                                      ,           PunctuationToken{tk: ","}
                                                                                                         $(,)?    MacroGroup{kind: "?"}
                                                                                                          (,)     MacroGroup.segments{dk: "()"}
                                                                                                           ,      PunctuationToken{tk: ","}*/
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    ($(#[$attr:meta])* pub enum $name:ident {                                                                                             /*
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration>
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     $(#[$attr:meta])*                            MacroGroup{kind: "*"}
      (#[$attr:meta])                             MacroGroup.segments{dk: "()"}
       #                                          PunctuationToken{tk: "#"}
        [$attr:meta]                              DelimGroup
         $attr:meta                               MacroParameterDeclaration
         $attr                                    McIdentifier
                                $name:ident       MacroParameterDeclaration
                                $name             McIdentifier
                                            {↲    <DelimGroup>                                                                            */
        $($(#[$var_attr:meta])* $variant:ident = $e:expr,)*                                                                               /*
        $($(#[$var_attr:meta])*•$variant:ident•=•$e:expr,)*    MacroGroup{kind: "*"}
         ($(#[$var_attr:meta])*•$variant:ident•=•$e:expr,)     MacroGroup.segments{dk: "()"}
          $(#[$var_attr:meta])*                                MacroGroup{kind: "*"}
           (#[$var_attr:meta])                                 MacroGroup.segments{dk: "()"}
            #                                                  PunctuationToken{tk: "#"}
             [$var_attr:meta]                                  DelimGroup
              $var_attr:meta                                   MacroParameterDeclaration
              $var_attr                                        McIdentifier
                                $variant:ident                 MacroParameterDeclaration
                                $variant                       McIdentifier
                                               =               PunctuationToken{tk: "="}
                                                 $e:expr       MacroParameterDeclaration
                                                 $e            McIdentifier
                                                        ,      PunctuationToken{tk: ","}                                                  */
    }) => {};                                                                                                                             /*
••••}           </DelimGroup>
••••})          </MacroRuleDeclaration.match>
          {}    MacroRuleDeclaration.transform{dk: "{}"}
••••})•=>•{}    </MacroRuleDeclaration>                                                                                                   */
    ($(#[$attr:meta])* pub enum $name:ident {                                                                                             /*
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration>
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     $(#[$attr:meta])*                            MacroGroup{kind: "*"}
      (#[$attr:meta])                             MacroGroup.segments{dk: "()"}
       #                                          PunctuationToken{tk: "#"}
        [$attr:meta]                              DelimGroup
         $attr:meta                               MacroParameterDeclaration
         $attr                                    McIdentifier
                                $name:ident       MacroParameterDeclaration
                                $name             McIdentifier
                                            {↲    <DelimGroup>                                                                            */
        $($(#[$var_attr:meta])* $variant:ident,)*                                                                                         /*
        $($(#[$var_attr:meta])*•$variant:ident,)*    MacroGroup{kind: "*"}
         ($(#[$var_attr:meta])*•$variant:ident,)     MacroGroup.segments{dk: "()"}
          $(#[$var_attr:meta])*                      MacroGroup{kind: "*"}
           (#[$var_attr:meta])                       MacroGroup.segments{dk: "()"}
            #                                        PunctuationToken{tk: "#"}
             [$var_attr:meta]                        DelimGroup
              $var_attr:meta                         MacroParameterDeclaration
              $var_attr                              McIdentifier
                                $variant:ident       MacroParameterDeclaration
                                $variant             McIdentifier
                                              ,      PunctuationToken{tk: ","}                                                            */
    }) => {};                                                                                                                             /*
••••}           </DelimGroup>
••••})          </MacroRuleDeclaration.match>
          {}    MacroRuleDeclaration.transform{dk: "{}"}
••••})•=>•{}    </MacroRuleDeclaration>                                                                                                   */
    (impl $fblock:tt [$($c:tt,)*] [$block:tt $(, $rest:tt)*]) => {};                                                                      /*
    (impl•$fblock:tt•[$($c:tt,)*]•[$block:tt•$(,•$rest:tt)*])•=>•{}    MacroRuleDeclaration
    (impl•$fblock:tt•[$($c:tt,)*]•[$block:tt•$(,•$rest:tt)*])          MacroRuleDeclaration.match{dk: "()"}
          $fblock:tt                                                   MacroParameterDeclaration
          $fblock                                                      McIdentifier
                     [$($c:tt,)*]                                      DelimGroup
                      $($c:tt,)*                                       MacroGroup{kind: "*"}
                       ($c:tt,)                                        MacroGroup.segments{dk: "()"}
                        $c:tt                                          MacroParameterDeclaration
                        $c                                             McIdentifier
                             ,                                         PunctuationToken{tk: ","}
                                  [$block:tt•$(,•$rest:tt)*]           DelimGroup
                                   $block:tt                           MacroParameterDeclaration
                                   $block                              McIdentifier
                                             $(,•$rest:tt)*            MacroGroup{kind: "*"}
                                              (,•$rest:tt)             MacroGroup.segments{dk: "()"}
                                               ,                       PunctuationToken{tk: ","}
                                                 $rest:tt              MacroParameterDeclaration
                                                 $rest                 McIdentifier
                                                                 {}    MacroRuleDeclaration.transform{dk: "{}"}                           */
    (impl $fblock:tt [$($blocks:tt,)*] []) => {};                                                                                         /*
    (impl•$fblock:tt•[$($blocks:tt,)*]•[])•=>•{}    MacroRuleDeclaration
    (impl•$fblock:tt•[$($blocks:tt,)*]•[])          MacroRuleDeclaration.match{dk: "()"}
          $fblock:tt                                MacroParameterDeclaration
          $fblock                                   McIdentifier
                     [$($blocks:tt,)*]              DelimGroup
                      $($blocks:tt,)*               MacroGroup{kind: "*"}
                       ($blocks:tt,)                MacroGroup.segments{dk: "()"}
                        $blocks:tt                  MacroParameterDeclaration
                        $blocks                     McIdentifier
                                  ,                 PunctuationToken{tk: ","}
                                       []           DelimGroup
                                              {}    MacroRuleDeclaration.transform{dk: "{}"}                                              */
    ($($ecode:ident: $message:expr,)* ; $($code:ident,)*) => {};                                                                          /*
    ($($ecode:ident:•$message:expr,)*•;•$($code:ident,)*)•=>•{}    MacroRuleDeclaration
    ($($ecode:ident:•$message:expr,)*•;•$($code:ident,)*)          MacroRuleDeclaration.match{dk: "()"}
     $($ecode:ident:•$message:expr,)*                              MacroGroup{kind: "*"}
      ($ecode:ident:•$message:expr,)                               MacroGroup.segments{dk: "()"}
       $ecode:ident                                                MacroParameterDeclaration
       $ecode                                                      McIdentifier
                   :                                               PunctuationToken{tk: ":"}
                     $message:expr                                 MacroParameterDeclaration
                     $message                                      McIdentifier
                                  ,                                PunctuationToken{tk: ","}
                                      ;                            PunctuationToken{tk: ";"}
                                        $($code:ident,)*           MacroGroup{kind: "*"}
                                         ($code:ident,)            MacroGroup.segments{dk: "()"}
                                          $code:ident              MacroParameterDeclaration
                                          $code                    McIdentifier
                                                     ,             PunctuationToken{tk: ","}
                                                             {}    MacroRuleDeclaration.transform{dk: "{}"}                               */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $(#[$attrs:meta])*                                                                                                                /*
        $(#[$attrs:meta])*    MacroGroup{kind: "*"}
         (#[$attrs:meta])     MacroGroup.segments{dk: "()"}
          #                   PunctuationToken{tk: "#"}
           [$attrs:meta]      DelimGroup
            $attrs:meta       MacroParameterDeclaration
            $attrs            McIdentifier                                                                                                */
        pub fn $n:ident(&self, $($name:ident: $ty:ty),* $(,)?) -> &Self                                                                   /*
               $n:ident                                                MacroParameterDeclaration
               $n                                                      McIdentifier
                       (&self,•$($name:ident:•$ty:ty),*•$(,)?)         DelimGroup
                        &                                              PunctuationToken{tk: "&"}
                             ,                                         PunctuationToken{tk: ","}
                               $($name:ident:•$ty:ty),*                MacroGroup{kind: "*"}
                                ($name:ident:•$ty:ty)                  MacroGroup.segments{dk: "()"}
                                 $name:ident                           MacroParameterDeclaration
                                 $name                                 McIdentifier
                                            :                          PunctuationToken{tk: ":"}
                                              $ty:ty                   MacroParameterDeclaration
                                              $ty                      McIdentifier
                                                     ,                 PunctuationToken{tk: ","}
                                                        $(,)?          MacroGroup{kind: "?"}
                                                         (,)           MacroGroup.segments{dk: "()"}
                                                          ,            PunctuationToken{tk: ","}
                                                               ->      PunctuationToken{tk: "->"}
                                                                  &    PunctuationToken{tk: "&"}                                          */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $enc:expr,                  // _______
/*      $enc:expr                                 MacroParameterDeclaration
        $enc                                      McIdentifier
                 ,                                PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
        $idx:expr,                  // _______
/*      $idx:expr                                 MacroParameterDeclaration
        $idx                                      McIdentifier
                 ,                                PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
        $struct:expr,               // _______
/*      $struct:expr                              MacroParameterDeclaration
        $struct                                   McIdentifier
                    ,                             PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
        $struct_name:ident,         // _______
/*      $struct_name:ident                        MacroParameterDeclaration
        $struct_name                              McIdentifier
                          ,                       PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
        [ $($name:ident),+$(,)? ],  // _______
/*      [•$($name:ident),+$(,)?•]                 DelimGroup
          $($name:ident),+                        MacroGroup{kind: "+"}
           ($name:ident)                          MacroGroup.segments{dk: "()"}
            $name:ident                           MacroParameterDeclaration
            $name                                 McIdentifier
                        ,                         PunctuationToken{tk: ","}
                          $(,)?                   MacroGroup{kind: "?"}
                           (,)                    MacroGroup.segments{dk: "()"}
                            ,                     PunctuationToken{tk: ","}
                                 ,                PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
        [ $($ignore:ident),+$(,)? ] // _______
/*      [•$($ignore:ident),+$(,)?•]               DelimGroup
          $($ignore:ident),+                      MacroGroup{kind: "+"}
           ($ignore:ident)                        MacroGroup.segments{dk: "()"}
            $ignore:ident                         MacroParameterDeclaration
            $ignore                               McIdentifier
                          ,                       PunctuationToken{tk: ","}
                            $(,)?                 MacroGroup{kind: "?"}
                             (,)                  MacroGroup.segments{dk: "()"}
                              ,                   PunctuationToken{tk: ","}                                                               */
                                    //•_______    Comment{line}
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $($Kind:ident($AstTy:ty) {                                                                                                        /*
        $($Kind:ident($AstTy:ty)•{↲    <MacroGroup{kind: "*"}>
         ($Kind:ident($AstTy:ty)•{↲    <MacroGroup.segments{dk: "()"}>
          $Kind:ident                  MacroParameterDeclaration
          $Kind                        McIdentifier
                     ($AstTy:ty)       DelimGroup
                      $AstTy:ty        MacroParameterDeclaration
                      $AstTy           McIdentifier
                                 {↲    <DelimGroup>                                                                                       */
            $kind_name:expr;                                                                                                              /*
            $kind_name:expr     MacroParameterDeclaration
            $kind_name          McIdentifier
                           ;    PunctuationToken{tk: ";"}                                                                                 */
            $(one fn $mut_visit_ast:ident; fn $visit_ast:ident;)?                                                                         /*
            $(one•fn•$mut_visit_ast:ident;•fn•$visit_ast:ident;)?    MacroGroup{kind: "?"}
             (one•fn•$mut_visit_ast:ident;•fn•$visit_ast:ident;)     MacroGroup.segments{dk: "()"}
                     $mut_visit_ast:ident                            MacroParameterDeclaration
                     $mut_visit_ast                                  McIdentifier
                                         ;                           PunctuationToken{tk: ";"}
                                              $visit_ast:ident       MacroParameterDeclaration
                                              $visit_ast             McIdentifier
                                                              ;      PunctuationToken{tk: ";"}                                            */
            $(many fn $flat_map_ast_elt:ident; fn $visit_ast_elt:ident($($args:tt)*);)?                                                   /*
            $(many•fn•$flat_map_ast_elt:ident;•fn•$visit_ast_elt:ident($($args:tt)*);)?    MacroGroup{kind: "?"}
             (many•fn•$flat_map_ast_elt:ident;•fn•$visit_ast_elt:ident($($args:tt)*);)     MacroGroup.segments{dk: "()"}
                      $flat_map_ast_elt:ident                                              MacroParameterDeclaration
                      $flat_map_ast_elt                                                    McIdentifier
                                             ;                                             PunctuationToken{tk: ";"}
                                                  $visit_ast_elt:ident                     MacroParameterDeclaration
                                                  $visit_ast_elt                           McIdentifier
                                                                      ($($args:tt)*)       DelimGroup
                                                                       $($args:tt)*        MacroGroup{kind: "*"}
                                                                        ($args:tt)         MacroGroup.segments{dk: "()"}
                                                                         $args:tt          MacroParameterDeclaration
                                                                         $args             McIdentifier
                                                                                    ;      PunctuationToken{tk: ";"}                      */
            fn $make_ast:ident;                                                                                                           /*
               $make_ast:ident     MacroParameterDeclaration
               $make_ast           McIdentifier
                              ;    PunctuationToken{tk: ";"}                                                                              */
        })*                                                                                                                               /*
••••••••}      </DelimGroup>
••••••••})     </MacroGroup.segments>
••••••••})*    </MacroGroup>                                                                                                              */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    ($ty:ident { $($field:ident $(: $value:expr)*),+ $(,)? }) => {};                                                                      /*
    ($ty:ident•{•$($field:ident•$(:•$value:expr)*),+•$(,)?•})•=>•{}    MacroRuleDeclaration
    ($ty:ident•{•$($field:ident•$(:•$value:expr)*),+•$(,)?•})          MacroRuleDeclaration.match{dk: "()"}
     $ty:ident                                                         MacroParameterDeclaration
     $ty                                                               McIdentifier
               {•$($field:ident•$(:•$value:expr)*),+•$(,)?•}           DelimGroup
                 $($field:ident•$(:•$value:expr)*),+                   MacroGroup{kind: "+"}
                  ($field:ident•$(:•$value:expr)*)                     MacroGroup.segments{dk: "()"}
                   $field:ident                                        MacroParameterDeclaration
                   $field                                              McIdentifier
                                $(:•$value:expr)*                      MacroGroup{kind: "*"}
                                 (:•$value:expr)                       MacroGroup.segments{dk: "()"}
                                  :                                    PunctuationToken{tk: ":"}
                                    $value:expr                        MacroParameterDeclaration
                                    $value                             McIdentifier
                                                  ,                    PunctuationToken{tk: ","}
                                                     $(,)?             MacroGroup{kind: "?"}
                                                      (,)              MacroGroup.segments{dk: "()"}
                                                       ,               PunctuationToken{tk: ","}
                                                                 {}    MacroRuleDeclaration.transform{dk: "{}"}                           */
    ($ty:ident::$method:ident($($value:expr),*)) => {};                                                                                   /*
    ($ty:ident::$method:ident($($value:expr),*))•=>•{}    MacroRuleDeclaration
    ($ty:ident::$method:ident($($value:expr),*))          MacroRuleDeclaration.match{dk: "()"}
     $ty:ident                                            MacroParameterDeclaration
     $ty                                                  McIdentifier
              ::                                          PunctuationToken{tk: "::"}
                $method:ident                             MacroParameterDeclaration
                $method                                   McIdentifier
                             ($($value:expr),*)           DelimGroup
                              $($value:expr),*            MacroGroup{kind: "*"}
                               ($value:expr)              MacroGroup.segments{dk: "()"}
                                $value:expr               MacroParameterDeclaration
                                $value                    McIdentifier
                                            ,             PunctuationToken{tk: ","}
                                                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                        */
    ($(                                                                                                                                   /*
    ($(↲    <MacroRuleDeclaration>
    ($(↲    <MacroRuleDeclaration.match{dk: "()"}>
     $(↲    <MacroGroup{kind: "+"}>
      (↲    <MacroGroup.segments{dk: "()"}>                                                                                               */
        $(#[doc = $doc:tt])* (accepted, $feature:ident, $ver:expr, $issue:expr, None),                                                    /*
        $(#[doc•=•$doc:tt])*                                                              MacroGroup{kind: "*"}
         (#[doc•=•$doc:tt])                                                               MacroGroup.segments{dk: "()"}
          #                                                                               PunctuationToken{tk: "#"}
           [doc•=•$doc:tt]                                                                DelimGroup
                =                                                                         PunctuationToken{tk: "="}
                  $doc:tt                                                                 MacroParameterDeclaration
                  $doc                                                                    McIdentifier
                             (accepted,•$feature:ident,•$ver:expr,•$issue:expr,•None)     DelimGroup
                                      ,                                                   PunctuationToken{tk: ","}
                                        $feature:ident                                    MacroParameterDeclaration
                                        $feature                                          McIdentifier
                                                      ,                                   PunctuationToken{tk: ","}
                                                        $ver:expr                         MacroParameterDeclaration
                                                        $ver                              McIdentifier
                                                                 ,                        PunctuationToken{tk: ","}
                                                                   $issue:expr            MacroParameterDeclaration
                                                                   $issue                 McIdentifier
                                                                              ,           PunctuationToken{tk: ","}
                                                                                     ,    PunctuationToken{tk: ","}                       */
    )+) => {};                                                                                                                            /*
••••)            </MacroGroup.segments>
••••)+           </MacroGroup>
••••)+)          </MacroRuleDeclaration.match>
           {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)+)•=>•{}    </MacroRuleDeclaration>                                                                                                  */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $( $(#[$attr:meta])* $variant:ident $($group:expr)?, $module:ident :: $name:ident, $method:ident, $target:expr, $generics:expr; )*/*
        $(•$(#[$attr:meta])*•$variant:ident•$($group:expr)?,•$module:ident•::•$name:ident,•$method:ident,•$target:expr,•$generics:expr;•)*    MacroGroup{kind: "*"}
         (•$(#[$attr:meta])*•$variant:ident•$($group:expr)?,•$module:ident•::•$name:ident,•$method:ident,•$target:expr,•$generics:expr;•)     MacroGroup.segments{dk: "()"}
           $(#[$attr:meta])*                                                                                                                  MacroGroup{kind: "*"}
            (#[$attr:meta])                                                                                                                   MacroGroup.segments{dk: "()"}
             #                                                                                                                                PunctuationToken{tk: "#"}
              [$attr:meta]                                                                                                                    DelimGroup
               $attr:meta                                                                                                                     MacroParameterDeclaration
               $attr                                                                                                                          McIdentifier
                             $variant:ident                                                                                                   MacroParameterDeclaration
                             $variant                                                                                                         McIdentifier
                                            $($group:expr)?                                                                                   MacroGroup{kind: "?"}
                                             ($group:expr)                                                                                    MacroGroup.segments{dk: "()"}
                                              $group:expr                                                                                     MacroParameterDeclaration
                                              $group                                                                                          McIdentifier
                                                           ,                                                                                  PunctuationToken{tk: ","}
                                                             $module:ident                                                                    MacroParameterDeclaration
                                                             $module                                                                          McIdentifier
                                                                           ::                                                                 PunctuationToken{tk: "::"}
                                                                              $name:ident                                                     MacroParameterDeclaration
                                                                              $name                                                           McIdentifier
                                                                                         ,                                                    PunctuationToken{tk: ","}
                                                                                           $method:ident                                      MacroParameterDeclaration
                                                                                           $method                                            McIdentifier
                                                                                                        ,                                     PunctuationToken{tk: ","}
                                                                                                          $target:expr                        MacroParameterDeclaration
                                                                                                          $target                             McIdentifier
                                                                                                                      ,                       PunctuationToken{tk: ","}
                                                                                                                        $generics:expr        MacroParameterDeclaration
                                                                                                                        $generics             McIdentifier
                                                                                                                                      ;       PunctuationToken{tk: ";"}*/
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    ([$($(#[$attr:meta])* fn $name:ident($($param:ident: $arg:ty),*);)*]) => {};                                                          /*
    ([$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])•=>•{}    MacroRuleDeclaration
    ([$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])          MacroRuleDeclaration.match{dk: "()"}
     [$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*]           DelimGroup
      $($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*            MacroGroup{kind: "*"}
       ($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)             MacroGroup.segments{dk: "()"}
        $(#[$attr:meta])*                                                          MacroGroup{kind: "*"}
         (#[$attr:meta])                                                           MacroGroup.segments{dk: "()"}
          #                                                                        PunctuationToken{tk: "#"}
           [$attr:meta]                                                            DelimGroup
            $attr:meta                                                             MacroParameterDeclaration
            $attr                                                                  McIdentifier
                             $name:ident                                           MacroParameterDeclaration
                             $name                                                 McIdentifier
                                        ($($param:ident:•$arg:ty),*)               DelimGroup
                                         $($param:ident:•$arg:ty),*                MacroGroup{kind: "*"}
                                          ($param:ident:•$arg:ty)                  MacroGroup.segments{dk: "()"}
                                           $param:ident                            MacroParameterDeclaration
                                           $param                                  McIdentifier
                                                       :                           PunctuationToken{tk: ":"}
                                                         $arg:ty                   MacroParameterDeclaration
                                                         $arg                      McIdentifier
                                                                 ,                 PunctuationToken{tk: ","}
                                                                    ;              PunctuationToken{tk: ";"}
                                                                             {}    MacroRuleDeclaration.transform{dk: "{}"}               */
    ([$hir:tt], [$($(#[$attr:meta])* fn $name:ident($($param:ident: $arg:ty),*);)*]) => {};                                               /*
    ([$hir:tt],•[$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])•=>•{}    MacroRuleDeclaration
    ([$hir:tt],•[$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])          MacroRuleDeclaration.match{dk: "()"}
     [$hir:tt]                                                                                DelimGroup
      $hir:tt                                                                                 MacroParameterDeclaration
      $hir                                                                                    McIdentifier
              ,                                                                               PunctuationToken{tk: ","}
                [$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*]           DelimGroup
                 $($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*            MacroGroup{kind: "*"}
                  ($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)             MacroGroup.segments{dk: "()"}
                   $(#[$attr:meta])*                                                          MacroGroup{kind: "*"}
                    (#[$attr:meta])                                                           MacroGroup.segments{dk: "()"}
                     #                                                                        PunctuationToken{tk: "#"}
                      [$attr:meta]                                                            DelimGroup
                       $attr:meta                                                             MacroParameterDeclaration
                       $attr                                                                  McIdentifier
                                        $name:ident                                           MacroParameterDeclaration
                                        $name                                                 McIdentifier
                                                   ($($param:ident:•$arg:ty),*)               DelimGroup
                                                    $($param:ident:•$arg:ty),*                MacroGroup{kind: "*"}
                                                     ($param:ident:•$arg:ty)                  MacroGroup.segments{dk: "()"}
                                                      $param:ident                            MacroParameterDeclaration
                                                      $param                                  McIdentifier
                                                                  :                           PunctuationToken{tk: ":"}
                                                                    $arg:ty                   MacroParameterDeclaration
                                                                    $arg                      McIdentifier
                                                                            ,                 PunctuationToken{tk: ","}
                                                                               ;              PunctuationToken{tk: ";"}
                                                                                        {}    MacroRuleDeclaration.transform{dk: "{}"}    */
    ([$span:expr, $($fmt:tt)*] $arg:expr, $($rest:tt)*) => {};                                                                            /*
    ([$span:expr,•$($fmt:tt)*]•$arg:expr,•$($rest:tt)*)•=>•{}    MacroRuleDeclaration
    ([$span:expr,•$($fmt:tt)*]•$arg:expr,•$($rest:tt)*)          MacroRuleDeclaration.match{dk: "()"}
     [$span:expr,•$($fmt:tt)*]                                   DelimGroup
      $span:expr                                                 MacroParameterDeclaration
      $span                                                      McIdentifier
                ,                                                PunctuationToken{tk: ","}
                  $($fmt:tt)*                                    MacroGroup{kind: "*"}
                   ($fmt:tt)                                     MacroGroup.segments{dk: "()"}
                    $fmt:tt                                      MacroParameterDeclaration
                    $fmt                                         McIdentifier
                               $arg:expr                         MacroParameterDeclaration
                               $arg                              McIdentifier
                                        ,                        PunctuationToken{tk: ","}
                                          $($rest:tt)*           MacroGroup{kind: "*"}
                                           ($rest:tt)            MacroGroup.segments{dk: "()"}
                                            $rest:tt             MacroParameterDeclaration
                                            $rest                McIdentifier
                                                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                 */
    ($($T:ty),*) => {};                                                                                                                   /*
    ($($T:ty),*)•=>•{}    MacroRuleDeclaration
    ($($T:ty),*)          MacroRuleDeclaration.match{dk: "()"}
     $($T:ty),*           MacroGroup{kind: "*"}
      ($T:ty)             MacroGroup.segments{dk: "()"}
       $T:ty              MacroParameterDeclaration
       $T                 McIdentifier
             ,            PunctuationToken{tk: ","}
                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                        */
    (# $var:ident) => {};                                                                                                                 /*
    (#•$var:ident)•=>•{}    MacroRuleDeclaration
    (#•$var:ident)          MacroRuleDeclaration.match{dk: "()"}
     #                      PunctuationToken{tk: "#"}
       $var:ident           MacroParameterDeclaration
       $var                 McIdentifier
                      {}    MacroRuleDeclaration.transform{dk: "{}"}                                                                      */
    ($call:ident! $extra:tt ($($b1:tt)*) ($($curr:tt)*)) => {};                                                                           /*
    ($call:ident!•$extra:tt•($($b1:tt)*)•($($curr:tt)*))•=>•{}    MacroRuleDeclaration
    ($call:ident!•$extra:tt•($($b1:tt)*)•($($curr:tt)*))          MacroRuleDeclaration.match{dk: "()"}
     $call:ident                                                  MacroParameterDeclaration
     $call                                                        McIdentifier
                !                                                 PunctuationToken{tk: "!"}
                  $extra:tt                                       MacroParameterDeclaration
                  $extra                                          McIdentifier
                            ($($b1:tt)*)                          DelimGroup
                             $($b1:tt)*                           MacroGroup{kind: "*"}
                              ($b1:tt)                            MacroGroup.segments{dk: "()"}
                               $b1:tt                             MacroParameterDeclaration
                               $b1                                McIdentifier
                                         ($($curr:tt)*)           DelimGroup
                                          $($curr:tt)*            MacroGroup{kind: "*"}
                                           ($curr:tt)             MacroGroup.segments{dk: "()"}
                                            $curr:tt              MacroParameterDeclaration
                                            $curr                 McIdentifier
                                                            {}    MacroRuleDeclaration.transform{dk: "{}"}                                */
    ($call:ident!($($extra:tt)*) # $var:ident) => {};                                                                                     /*
    ($call:ident!($($extra:tt)*)•#•$var:ident)•=>•{}    MacroRuleDeclaration
    ($call:ident!($($extra:tt)*)•#•$var:ident)          MacroRuleDeclaration.match{dk: "()"}
     $call:ident                                        MacroParameterDeclaration
     $call                                              McIdentifier
                !                                       PunctuationToken{tk: "!"}
                 ($($extra:tt)*)                        DelimGroup
                  $($extra:tt)*                         MacroGroup{kind: "*"}
                   ($extra:tt)                          MacroGroup.segments{dk: "()"}
                    $extra:tt                           MacroParameterDeclaration
                    $extra                              McIdentifier
                                 #                      PunctuationToken{tk: "#"}
                                   $var:ident           MacroParameterDeclaration
                                   $var                 McIdentifier
                                                  {}    MacroRuleDeclaration.transform{dk: "{}"}                                          */
    ($tokens:ident                                                                                                                        /*
    ($tokens:ident↲    <MacroRuleDeclaration>
    ($tokens:ident↲    <MacroRuleDeclaration.match{dk: "()"}>
     $tokens:ident     MacroParameterDeclaration
     $tokens           McIdentifier                                                                                                       */
        ($($b3:tt)*) ($($b2:tt)*) ($($b1:tt)*)                                                                                            /*
        ($($b3:tt)*)                              DelimGroup
         $($b3:tt)*                               MacroGroup{kind: "*"}
          ($b3:tt)                                MacroGroup.segments{dk: "()"}
           $b3:tt                                 MacroParameterDeclaration
           $b3                                    McIdentifier
                     ($($b2:tt)*)                 DelimGroup
                      $($b2:tt)*                  MacroGroup{kind: "*"}
                       ($b2:tt)                   MacroGroup.segments{dk: "()"}
                        $b2:tt                    MacroParameterDeclaration
                        $b2                       McIdentifier
                                  ($($b1:tt)*)    DelimGroup
                                   $($b1:tt)*     MacroGroup{kind: "*"}
                                    ($b1:tt)      MacroGroup.segments{dk: "()"}
                                     $b1:tt       MacroParameterDeclaration
                                     $b1          McIdentifier                                                                            */
        ($($curr:tt)*)                                                                                                                    /*
        ($($curr:tt)*)    DelimGroup
         $($curr:tt)*     MacroGroup{kind: "*"}
          ($curr:tt)      MacroGroup.segments{dk: "()"}
           $curr:tt       MacroParameterDeclaration
           $curr          McIdentifier                                                                                                    */
        ($($a1:tt)*) ($($a2:tt)*) ($($a3:tt)*)                                                                                            /*
        ($($a1:tt)*)                              DelimGroup
         $($a1:tt)*                               MacroGroup{kind: "*"}
          ($a1:tt)                                MacroGroup.segments{dk: "()"}
           $a1:tt                                 MacroParameterDeclaration
           $a1                                    McIdentifier
                     ($($a2:tt)*)                 DelimGroup
                      $($a2:tt)*                  MacroGroup{kind: "*"}
                       ($a2:tt)                   MacroGroup.segments{dk: "()"}
                        $a2:tt                    MacroParameterDeclaration
                        $a2                       McIdentifier
                                  ($($a3:tt)*)    DelimGroup
                                   $($a3:tt)*     MacroGroup{kind: "*"}
                                    ($a3:tt)      MacroGroup.segments{dk: "()"}
                                     $a3:tt       MacroParameterDeclaration
                                     $a3          McIdentifier                                                                            */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    ($tokens:ident $b3:tt $b2:tt $b1:tt @ $a1:tt $a2:tt $a3:tt) => {};                                                                    /*
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•@•$a1:tt•$a2:tt•$a3:tt)•=>•{}    MacroRuleDeclaration
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•@•$a1:tt•$a2:tt•$a3:tt)          MacroRuleDeclaration.match{dk: "()"}
     $tokens:ident                                                       MacroParameterDeclaration
     $tokens                                                             McIdentifier
                   $b3:tt                                                MacroParameterDeclaration
                   $b3                                                   McIdentifier
                          $b2:tt                                         MacroParameterDeclaration
                          $b2                                            McIdentifier
                                 $b1:tt                                  MacroParameterDeclaration
                                 $b1                                     McIdentifier
                                        @                                PunctuationToken{tk: "@"}
                                          $a1:tt                         MacroParameterDeclaration
                                          $a1                            McIdentifier
                                                 $a2:tt                  MacroParameterDeclaration
                                                 $a2                     McIdentifier
                                                        $a3:tt           MacroParameterDeclaration
                                                        $a3              McIdentifier
                                                                   {}    MacroRuleDeclaration.transform{dk: "{}"}                         */
    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) * $a3:tt) => {};                                                            /*
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•(#)•(•$($inner:tt)*•)•*•$a3:tt)•=>•{}    MacroRuleDeclaration
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•(#)•(•$($inner:tt)*•)•*•$a3:tt)          MacroRuleDeclaration.match{dk: "()"}
     $tokens:ident                                                               MacroParameterDeclaration
     $tokens                                                                     McIdentifier
                   $b3:tt                                                        MacroParameterDeclaration
                   $b3                                                           McIdentifier
                          $b2:tt                                                 MacroParameterDeclaration
                          $b2                                                    McIdentifier
                                 $b1:tt                                          MacroParameterDeclaration
                                 $b1                                             McIdentifier
                                        (#)                                      DelimGroup
                                         #                                       PunctuationToken{tk: "#"}
                                            (•$($inner:tt)*•)                    DelimGroup
                                              $($inner:tt)*                      MacroGroup{kind: "*"}
                                               ($inner:tt)                       MacroGroup.segments{dk: "()"}
                                                $inner:tt                        MacroParameterDeclaration
                                                $inner                           McIdentifier
                                                              *                  PunctuationToken{tk: "*"}
                                                                $a3:tt           MacroParameterDeclaration
                                                                $a3              McIdentifier
                                                                           {}    MacroRuleDeclaration.transform{dk: "{}"}                 */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        [$($attrs_pub:tt)*]                                                                                                               /*
        [$($attrs_pub:tt)*]    DelimGroup
         $($attrs_pub:tt)*     MacroGroup{kind: "*"}
          ($attrs_pub:tt)      MacroGroup.segments{dk: "()"}
           $attrs_pub:tt       MacroParameterDeclaration
           $attrs_pub          McIdentifier                                                                                               */
        enum $name:ident #no_visit $($rest:tt)*                                                                                           /*
             $name:ident                           MacroParameterDeclaration
             $name                                 McIdentifier
                         #                         PunctuationToken{tk: "#"}
                                   $($rest:tt)*    MacroGroup{kind: "*"}
                                    ($rest:tt)     MacroGroup.segments{dk: "()"}
                                     $rest:tt      MacroParameterDeclaration
                                     $rest         McIdentifier                                                                           */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $(#[$enum_attr:meta])*                                                                                                            /*
        $(#[$enum_attr:meta])*    MacroGroup{kind: "*"}
         (#[$enum_attr:meta])     MacroGroup.segments{dk: "()"}
          #                       PunctuationToken{tk: "#"}
           [$enum_attr:meta]      DelimGroup
            $enum_attr:meta       MacroParameterDeclaration
            $enum_attr            McIdentifier                                                                                            */
        $pub:ident $enum:ident $name:ident #$tag:ident $body:tt                                                                           /*
        $pub:ident                                                 MacroParameterDeclaration
        $pub                                                       McIdentifier
                   $enum:ident                                     MacroParameterDeclaration
                   $enum                                           McIdentifier
                               $name:ident                         MacroParameterDeclaration
                               $name                               McIdentifier
                                           #                       PunctuationToken{tk: "#"}
                                            $tag:ident             MacroParameterDeclaration
                                            $tag                   McIdentifier
                                                       $body:tt    MacroParameterDeclaration
                                                       $body       McIdentifier                                                           */
        $($remaining:tt)*                                                                                                                 /*
        $($remaining:tt)*    MacroGroup{kind: "*"}
         ($remaining:tt)     MacroGroup.segments{dk: "()"}
          $remaining:tt      MacroParameterDeclaration
          $remaining         McIdentifier                                                                                                 */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>
    (↲    <MacroRuleDeclaration.match{dk: "()"}>                                                                                          */
        $pub:ident $enum:ident $name:ident {                                                                                              /*
        $pub:ident                               MacroParameterDeclaration
        $pub                                     McIdentifier
                   $enum:ident                   MacroParameterDeclaration
                   $enum                         McIdentifier
                               $name:ident       MacroParameterDeclaration
                               $name             McIdentifier
                                           {↲    <DelimGroup>                                                                             */
            $(                                                                                                                            /*
            $(↲    <MacroGroup{kind: "*"}>
             (↲    <MacroGroup.segments{dk: "()"}>                                                                                        */
                $(#[$variant_attr:meta])*                                                                                                 /*
                $(#[$variant_attr:meta])*    MacroGroup{kind: "*"}
                 (#[$variant_attr:meta])     MacroGroup.segments{dk: "()"}
                  #                          PunctuationToken{tk: "#"}
                   [$variant_attr:meta]      DelimGroup
                    $variant_attr:meta       MacroParameterDeclaration
                    $variant_attr            McIdentifier                                                                                 */
                $variant:ident $( ($($member:ident)::+) )*,                                                                               /*
                $variant:ident                                 MacroParameterDeclaration
                $variant                                       McIdentifier
                               $(•($($member:ident)::+)•)*     MacroGroup{kind: "*"}
                                (•($($member:ident)::+)•)      MacroGroup.segments{dk: "()"}
                                  ($($member:ident)::+)        DelimGroup
                                   $($member:ident)::+         MacroGroup{kind: "+"}
                                    ($member:ident)            MacroGroup.segments{dk: "()"}
                                     $member:ident             MacroParameterDeclaration
                                     $member                   McIdentifier
                                                   ::          PunctuationToken{tk: "::"}
                                                          ,    PunctuationToken{tk: ","}                                                  */
            )*                                                                                                                            /*
••••••••••••)     </MacroGroup.segments>
••••••••••••)*    </MacroGroup>                                                                                                           */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */

        $($remaining:tt)*                                                                                                                 /*
        $($remaining:tt)*    MacroGroup{kind: "*"}
         ($remaining:tt)     MacroGroup.segments{dk: "()"}
          $remaining:tt      MacroParameterDeclaration
          $remaining         McIdentifier                                                                                                 */
    ) => {};                                                                                                                              /*
••••)          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••)•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (($($arms:tt)*) $tokens:ident $name:ident { $variant:ident $member:ident, $($next:tt)*}) => {};                                       /*
    (($($arms:tt)*)•$tokens:ident•$name:ident•{•$variant:ident•$member:ident,•$($next:tt)*})•=>•{}    MacroRuleDeclaration
    (($($arms:tt)*)•$tokens:ident•$name:ident•{•$variant:ident•$member:ident,•$($next:tt)*})          MacroRuleDeclaration.match{dk: "()"}
     ($($arms:tt)*)                                                                                   DelimGroup
      $($arms:tt)*                                                                                    MacroGroup{kind: "*"}
       ($arms:tt)                                                                                     MacroGroup.segments{dk: "()"}
        $arms:tt                                                                                      MacroParameterDeclaration
        $arms                                                                                         McIdentifier
                    $tokens:ident                                                                     MacroParameterDeclaration
                    $tokens                                                                           McIdentifier
                                  $name:ident                                                         MacroParameterDeclaration
                                  $name                                                               McIdentifier
                                              {•$variant:ident•$member:ident,•$($next:tt)*}           DelimGroup
                                                $variant:ident                                        MacroParameterDeclaration
                                                $variant                                              McIdentifier
                                                               $member:ident                          MacroParameterDeclaration
                                                               $member                                McIdentifier
                                                                            ,                         PunctuationToken{tk: ","}
                                                                              $($next:tt)*            MacroGroup{kind: "*"}
                                                                               ($next:tt)             MacroGroup.segments{dk: "()"}
                                                                                $next:tt              MacroParameterDeclaration
                                                                                $next                 McIdentifier
                                                                                                {}    MacroRuleDeclaration.transform{dk: "{}"}*/
    ($mac:ident!($(#[$m:meta])* $pub:ident $($t:tt)*)) => {};                                                                             /*
    ($mac:ident!($(#[$m:meta])*•$pub:ident•$($t:tt)*))•=>•{}    MacroRuleDeclaration
    ($mac:ident!($(#[$m:meta])*•$pub:ident•$($t:tt)*))          MacroRuleDeclaration.match{dk: "()"}
     $mac:ident                                                 MacroParameterDeclaration
     $mac                                                       McIdentifier
               !                                                PunctuationToken{tk: "!"}
                ($(#[$m:meta])*•$pub:ident•$($t:tt)*)           DelimGroup
                 $(#[$m:meta])*                                 MacroGroup{kind: "*"}
                  (#[$m:meta])                                  MacroGroup.segments{dk: "()"}
                   #                                            PunctuationToken{tk: "#"}
                    [$m:meta]                                   DelimGroup
                     $m:meta                                    MacroParameterDeclaration
                     $m                                         McIdentifier
                                $pub:ident                      MacroParameterDeclaration
                                $pub                            McIdentifier
                                           $($t:tt)*            MacroGroup{kind: "*"}
                                            ($t:tt)             MacroGroup.segments{dk: "()"}
                                             $t:tt              MacroParameterDeclaration
                                             $t                 McIdentifier
                                                          {}    MacroRuleDeclaration.transform{dk: "{}"}                                  */
    ($($token:tt pub struct $name:ident #[$doc:meta])*) => {};                                                                            /*
    ($($token:tt•pub•struct•$name:ident•#[$doc:meta])*)•=>•{}    MacroRuleDeclaration
    ($($token:tt•pub•struct•$name:ident•#[$doc:meta])*)          MacroRuleDeclaration.match{dk: "()"}
     $($token:tt•pub•struct•$name:ident•#[$doc:meta])*           MacroGroup{kind: "*"}
      ($token:tt•pub•struct•$name:ident•#[$doc:meta])            MacroGroup.segments{dk: "()"}
       $token:tt                                                 MacroParameterDeclaration
       $token                                                    McIdentifier
                            $name:ident                          MacroParameterDeclaration
                            $name                                McIdentifier
                                        #                        PunctuationToken{tk: "#"}
                                         [$doc:meta]             DelimGroup
                                          $doc:meta              MacroParameterDeclaration
                                          $doc                   McIdentifier
                                                           {}    MacroRuleDeclaration.transform{dk: "{}"}                                 */
    ($($token:tt pub struct $name:ident/$len:tt #[$doc:meta])*) => {};                                                                    /*
    ($($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])*)•=>•{}    MacroRuleDeclaration
    ($($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])*)          MacroRuleDeclaration.match{dk: "()"}
     $($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])*           MacroGroup{kind: "*"}
      ($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])            MacroGroup.segments{dk: "()"}
       $token:tt                                                         MacroParameterDeclaration
       $token                                                            McIdentifier
                            $name:ident                                  MacroParameterDeclaration
                            $name                                        McIdentifier
                                       /                                 PunctuationToken{tk: "/"}
                                        $len:tt                          MacroParameterDeclaration
                                        $len                             McIdentifier
                                                #                        PunctuationToken{tk: "#"}
                                                 [$doc:meta]             DelimGroup
                                                  $doc:meta              MacroParameterDeclaration
                                                  $doc                   McIdentifier
                                                                   {}    MacroRuleDeclaration.transform{dk: "{}"}                         */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>
    {↲    <MacroRuleDeclaration.match{dk: "{}"}>                                                                                          */
        $($name:ident)::+ $(<$param:ident>)?                                                                                              /*
        $($name:ident)::+                       MacroGroup{kind: "+"}
         ($name:ident)                          MacroGroup.segments{dk: "()"}
          $name:ident                           MacroParameterDeclaration
          $name                                 McIdentifier
                      ::                        PunctuationToken{tk: "::"}
                          $(<$param:ident>)?    MacroGroup{kind: "?"}
                           (<$param:ident>)     MacroGroup.segments{dk: "()"}
                            <                   PunctuationToken{tk: "<"}
                             $param:ident       MacroParameterDeclaration
                             $param             McIdentifier
                                         >      PunctuationToken{tk: ">"}                                                                 */
        $([$field:tt $this:ident $other:ident])*                                                                                          /*
        $([$field:tt•$this:ident•$other:ident])*    MacroGroup{kind: "*"}
         ([$field:tt•$this:ident•$other:ident])     MacroGroup.segments{dk: "()"}
          [$field:tt•$this:ident•$other:ident]      DelimGroup
           $field:tt                                MacroParameterDeclaration
           $field                                   McIdentifier
                     $this:ident                    MacroParameterDeclaration
                     $this                          McIdentifier
                                 $other:ident       MacroParameterDeclaration
                                 $other             McIdentifier                                                                          */
        $(![$ignore:tt])*;                                                                                                                /*
        $(![$ignore:tt])*     MacroGroup{kind: "*"}
         (![$ignore:tt])      MacroGroup.segments{dk: "()"}
          !                   PunctuationToken{tk: "!"}
           [$ignore:tt]       DelimGroup
            $ignore:tt        MacroParameterDeclaration
            $ignore           McIdentifier
                         ;    PunctuationToken{tk: ";"}                                                                                   */
        !$next:tt                                                                                                                         /*
        !            PunctuationToken{tk: "!"}
         $next:tt    MacroParameterDeclaration
         $next       McIdentifier                                                                                                         */
        $($rest:tt)*                                                                                                                      /*
        $($rest:tt)*    MacroGroup{kind: "*"}
         ($rest:tt)     MacroGroup.segments{dk: "()"}
          $rest:tt      MacroParameterDeclaration
          $rest         McIdentifier                                                                                                      */
    } => {};                                                                                                                              /*
••••}          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••}•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>
    {↲    <MacroRuleDeclaration.match{dk: "{}"}>                                                                                          */
        $($name:ident)::+;                                                                                                                /*
        $($name:ident)::+     MacroGroup{kind: "+"}
         ($name:ident)        MacroGroup.segments{dk: "()"}
          $name:ident         MacroParameterDeclaration
          $name               McIdentifier
                      ::      PunctuationToken{tk: "::"}
                         ;    PunctuationToken{tk: ";"}                                                                                   */
        $([$($variant:ident)::+; $([$field:tt $this:ident $other:ident])* $(![$ignore:tt])*])*                                            /*
        $([$($variant:ident)::+;•$([$field:tt•$this:ident•$other:ident])*•$(![$ignore:tt])*])*    MacroGroup{kind: "*"}
         ([$($variant:ident)::+;•$([$field:tt•$this:ident•$other:ident])*•$(![$ignore:tt])*])     MacroGroup.segments{dk: "()"}
          [$($variant:ident)::+;•$([$field:tt•$this:ident•$other:ident])*•$(![$ignore:tt])*]      DelimGroup
           $($variant:ident)::+                                                                   MacroGroup{kind: "+"}
            ($variant:ident)                                                                      MacroGroup.segments{dk: "()"}
             $variant:ident                                                                       MacroParameterDeclaration
             $variant                                                                             McIdentifier
                            ::                                                                    PunctuationToken{tk: "::"}
                               ;                                                                  PunctuationToken{tk: ";"}
                                 $([$field:tt•$this:ident•$other:ident])*                         MacroGroup{kind: "*"}
                                  ([$field:tt•$this:ident•$other:ident])                          MacroGroup.segments{dk: "()"}
                                   [$field:tt•$this:ident•$other:ident]                           DelimGroup
                                    $field:tt                                                     MacroParameterDeclaration
                                    $field                                                        McIdentifier
                                              $this:ident                                         MacroParameterDeclaration
                                              $this                                               McIdentifier
                                                          $other:ident                            MacroParameterDeclaration
                                                          $other                                  McIdentifier
                                                                          $(![$ignore:tt])*       MacroGroup{kind: "*"}
                                                                           (![$ignore:tt])        MacroGroup.segments{dk: "()"}
                                                                            !                     PunctuationToken{tk: "!"}
                                                                             [$ignore:tt]         DelimGroup
                                                                              $ignore:tt          MacroParameterDeclaration
                                                                              $ignore             McIdentifier                            */
    } => {};                                                                                                                              /*
••••}          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••}•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>
    {↲    <MacroRuleDeclaration.match{dk: "{}"}>                                                                                          */
        $($name:ident)::+;                                                                                                                /*
        $($name:ident)::+     MacroGroup{kind: "+"}
         ($name:ident)        MacroGroup.segments{dk: "()"}
          $name:ident         MacroParameterDeclaration
          $name               McIdentifier
                      ::      PunctuationToken{tk: "::"}
                         ;    PunctuationToken{tk: ";"}                                                                                   */
        $([$($variant:ident)::+; $($fields:tt)*])*                                                                                        /*
        $([$($variant:ident)::+;•$($fields:tt)*])*    MacroGroup{kind: "*"}
         ([$($variant:ident)::+;•$($fields:tt)*])     MacroGroup.segments{dk: "()"}
          [$($variant:ident)::+;•$($fields:tt)*]      DelimGroup
           $($variant:ident)::+                       MacroGroup{kind: "+"}
            ($variant:ident)                          MacroGroup.segments{dk: "()"}
             $variant:ident                           MacroParameterDeclaration
             $variant                                 McIdentifier
                            ::                        PunctuationToken{tk: "::"}
                               ;                      PunctuationToken{tk: ";"}
                                 $($fields:tt)*       MacroGroup{kind: "*"}
                                  ($fields:tt)        MacroGroup.segments{dk: "()"}
                                   $fields:tt         MacroParameterDeclaration
                                   $fields            McIdentifier                                                                        */
        $next:ident                                                                                                                       /*
        $next:ident    MacroParameterDeclaration
        $next          McIdentifier                                                                                                       */
        $($rest:tt)*                                                                                                                      /*
        $($rest:tt)*    MacroGroup{kind: "*"}
         ($rest:tt)     MacroGroup.segments{dk: "()"}
          $rest:tt      MacroParameterDeclaration
          $rest         McIdentifier                                                                                                      */
    } => {};                                                                                                                              /*
••••}          </MacroRuleDeclaration.match>
         {}    MacroRuleDeclaration.transform{dk: "{}"}
••••}•=>•{}    </MacroRuleDeclaration>                                                                                                    */
    (($expr:ident) as $t:ty, @$snapshot:literal) => {};                                                                                   /*
    (($expr:ident)•as•$t:ty,•@$snapshot:literal)•=>•{}    MacroRuleDeclaration
    (($expr:ident)•as•$t:ty,•@$snapshot:literal)          MacroRuleDeclaration.match{dk: "()"}
     ($expr:ident)                                        DelimGroup
      $expr:ident                                         MacroParameterDeclaration
      $expr                                               McIdentifier
                      $t:ty                               MacroParameterDeclaration
                      $t                                  McIdentifier
                           ,                              PunctuationToken{tk: ","}
                             @                            PunctuationToken{tk: "@"}
                              $snapshot:literal           MacroParameterDeclaration
                              $snapshot                   McIdentifier
                                                    {}    MacroRuleDeclaration.transform{dk: "{}"}                                        */
	($(#[$smeta:meta])* pub struct $stratname:ident [$($sgen:tt)*][$($swhere:tt)*]                                                        /*
	($(#[$smeta:meta])*•pub•struct•$stratname:ident•[$($sgen:tt)*][$($swhere:tt)*]•↲    <MacroRuleDeclaration>
	($(#[$smeta:meta])*•pub•struct•$stratname:ident•[$($sgen:tt)*][$($swhere:tt)*]•↲    <MacroRuleDeclaration.match{dk: "()"}>
	 $(#[$smeta:meta])*                                                                 MacroGroup{kind: "*"}
	  (#[$smeta:meta])                                                                  MacroGroup.segments{dk: "()"}
	   #                                                                                PunctuationToken{tk: "#"}
	    [$smeta:meta]                                                                   DelimGroup
	     $smeta:meta                                                                    MacroParameterDeclaration
	     $smeta                                                                         McIdentifier
	                               $stratname:ident                                     MacroParameterDeclaration
	                               $stratname                                           McIdentifier
	                                                [$($sgen:tt)*]                      DelimGroup
	                                                 $($sgen:tt)*                       MacroGroup{kind: "*"}
	                                                  ($sgen:tt)                        MacroGroup.segments{dk: "()"}
	                                                   $sgen:tt                         MacroParameterDeclaration
	                                                   $sgen                            McIdentifier
	                                                              [$($swhere:tt)*]      DelimGroup
	                                                               $($swhere:tt)*       MacroGroup{kind: "*"}
	                                                                ($swhere:tt)        MacroGroup.segments{dk: "()"}
	                                                                 $swhere:tt         MacroParameterDeclaration
	                                                                 $swhere            McIdentifier                                      */
	($innerstrat:ty) -> $stratvtty:ty; $(#[$vmeta:meta])* pub struct $vtname:ident                                                        /*
	($innerstrat:ty)                                                                  DelimGroup
	 $innerstrat:ty                                                                   MacroParameterDeclaration
	 $innerstrat                                                                      McIdentifier
	                 ->                                                               PunctuationToken{tk: "->"}
	                    $stratvtty:ty                                                 MacroParameterDeclaration
	                    $stratvtty                                                    McIdentifier
	                                 ;                                                PunctuationToken{tk: ";"}
	                                   $(#[$vmeta:meta])*                             MacroGroup{kind: "*"}
	                                    (#[$vmeta:meta])                              MacroGroup.segments{dk: "()"}
	                                     #                                            PunctuationToken{tk: "#"}
	                                      [$vmeta:meta]                               DelimGroup
	                                       $vmeta:meta                                MacroParameterDeclaration
	                                       $vmeta                                     McIdentifier
	                                                                 $vtname:ident    MacroParameterDeclaration
	                                                                 $vtname          McIdentifier                                        */
	[$($vgen:tt)*][$($vwhere:tt)*] ($innervt:ty) -> $actualty:ty; ) => {}                                                                 /*
	[$($vgen:tt)*]                                                           DelimGroup
	 $($vgen:tt)*                                                            MacroGroup{kind: "*"}
	  ($vgen:tt)                                                             MacroGroup.segments{dk: "()"}
	   $vgen:tt                                                              MacroParameterDeclaration
	   $vgen                                                                 McIdentifier
	              [$($vwhere:tt)*]                                           DelimGroup
	               $($vwhere:tt)*                                            MacroGroup{kind: "*"}
	                ($vwhere:tt)                                             MacroGroup.segments{dk: "()"}
	                 $vwhere:tt                                              MacroParameterDeclaration
	                 $vwhere                                                 McIdentifier
	                               ($innervt:ty)                             DelimGroup
	                                $innervt:ty                              MacroParameterDeclaration
	                                $innervt                                 McIdentifier
	                                             ->                          PunctuationToken{tk: "->"}
	                                                $actualty:ty             MacroParameterDeclaration
	                                                $actualty                McIdentifier
	                                                            ;            PunctuationToken{tk: ";"}
   ╚[$($vgen:tt)*][$($vwhere:tt)*]•($innervt:ty)•->•$actualty:ty;•)          </MacroRuleDeclaration.match>
	                                                                   {}    MacroRuleDeclaration.transform{dk: "{}"}
   ╚[$($vgen:tt)*][$($vwhere:tt)*]•($innervt:ty)•->•$actualty:ty;•)•=>•{}    </MacroRuleDeclaration>                                      */
    
    { $ head : expr } => { Cons ( $ head , Nil ) } ;                                                                                      /*
    {•$•head•:•expr•}•=>•{•Cons•(•$•head•,•Nil•)•}    MacroRuleDeclaration
    {•$•head•:•expr•}                                 MacroRuleDeclaration.match{dk: "{}"}
      $                                               PunctuationToken{tk: "$"}
             :                                        PunctuationToken{tk: ":"}
                         {•Cons•(•$•head•,•Nil•)•}    MacroRuleDeclaration.transform{dk: "{}"}
                                (•$•head•,•Nil•)      DelimGroup
                                  $                   PunctuationToken{tk: "$"}
                                         ,            PunctuationToken{tk: ","}                                                           */
    { $ head : expr , $ ( $ tail : expr ) , * } => { Cons ( $ head , hlist ! ( $ ( $ tail ) , * ) ) } ;                                   /*
    {•$•head•:•expr•,•$•(•$•tail•:•expr•)•,•*•}•=>•{•Cons•(•$•head•,•hlist•!•(•$•(•$•tail•)•,•*•)•)•}    MacroRuleDeclaration
    {•$•head•:•expr•,•$•(•$•tail•:•expr•)•,•*•}                                                          MacroRuleDeclaration.match{dk: "{}"}
      $                                                                                                  PunctuationToken{tk: "$"}
             :                                                                                           PunctuationToken{tk: ":"}
                    ,                                                                                    PunctuationToken{tk: ","}
                      $                                                                                  PunctuationToken{tk: "$"}
                        (•$•tail•:•expr•)                                                                DelimGroup
                          $                                                                              PunctuationToken{tk: "$"}
                                 :                                                                       PunctuationToken{tk: ":"}
                                          ,                                                              PunctuationToken{tk: ","}
                                            *                                                            PunctuationToken{tk: "*"}
                                                   {•Cons•(•$•head•,•hlist•!•(•$•(•$•tail•)•,•*•)•)•}    MacroRuleDeclaration.transform{dk: "{}"}
                                                          (•$•head•,•hlist•!•(•$•(•$•tail•)•,•*•)•)      DelimGroup
                                                            $                                            PunctuationToken{tk: "$"}
                                                                   ,                                     PunctuationToken{tk: ","}
                                                                           !                             PunctuationToken{tk: "!"}
                                                                             (•$•(•$•tail•)•,•*•)        DelimGroup
                                                                               $                         PunctuationToken{tk: "$"}
                                                                                 (•$•tail•)              DelimGroup
                                                                                   $                     PunctuationToken{tk: "$"}
                                                                                            ,            PunctuationToken{tk: ","}
                                                                                              *          PunctuationToken{tk: "*"}        */
    { $ head : ty } => { Cons < $ head , Nil > } ;                                                                                        /*
    {•$•head•:•ty•}•=>•{•Cons•<•$•head•,•Nil•>•}    MacroRuleDeclaration
    {•$•head•:•ty•}                                 MacroRuleDeclaration.match{dk: "{}"}
      $                                             PunctuationToken{tk: "$"}
             :                                      PunctuationToken{tk: ":"}
                       {•Cons•<•$•head•,•Nil•>•}    MacroRuleDeclaration.transform{dk: "{}"}
                              <                     PunctuationToken{tk: "<"}
                                $                   PunctuationToken{tk: "$"}
                                       ,            PunctuationToken{tk: ","}
                                             >      PunctuationToken{tk: ">"}                                                             */
    { $ head : ty , $ ( $ tail : ty ) , * } => { Cons < $ head , HList ! ( $ ( $ tail ) , * ) > } ;                                       /*
    {•$•head•:•ty•,•$•(•$•tail•:•ty•)•,•*•}•=>•{•Cons•<•$•head•,•HList•!•(•$•(•$•tail•)•,•*•)•>•}    MacroRuleDeclaration
    {•$•head•:•ty•,•$•(•$•tail•:•ty•)•,•*•}                                                          MacroRuleDeclaration.match{dk: "{}"}
      $                                                                                              PunctuationToken{tk: "$"}
             :                                                                                       PunctuationToken{tk: ":"}
                  ,                                                                                  PunctuationToken{tk: ","}
                    $                                                                                PunctuationToken{tk: "$"}
                      (•$•tail•:•ty•)                                                                DelimGroup
                        $                                                                            PunctuationToken{tk: "$"}
                               :                                                                     PunctuationToken{tk: ":"}
                                      ,                                                              PunctuationToken{tk: ","}
                                        *                                                            PunctuationToken{tk: "*"}
                                               {•Cons•<•$•head•,•HList•!•(•$•(•$•tail•)•,•*•)•>•}    MacroRuleDeclaration.transform{dk: "{}"}
                                                      <                                              PunctuationToken{tk: "<"}
                                                        $                                            PunctuationToken{tk: "$"}
                                                               ,                                     PunctuationToken{tk: ","}
                                                                       !                             PunctuationToken{tk: "!"}
                                                                         (•$•(•$•tail•)•,•*•)        DelimGroup
                                                                           $                         PunctuationToken{tk: "$"}
                                                                             (•$•tail•)              DelimGroup
                                                                               $                     PunctuationToken{tk: "$"}
                                                                                        ,            PunctuationToken{tk: ","}
                                                                                          *          PunctuationToken{tk: "*"}
                                                                                              >      PunctuationToken{tk: ">"}            */
    { ( $ ( $ LHS : tt ) + ) } => { Expr ! ( $ ( $ LHS ) + ) } ;                                                                          /*
    {•(•$•(•$•LHS•:•tt•)•+•)•}•=>•{•Expr•!•(•$•(•$•LHS•)•+•)•}    MacroRuleDeclaration
    {•(•$•(•$•LHS•:•tt•)•+•)•}                                    MacroRuleDeclaration.match{dk: "{}"}
      (•$•(•$•LHS•:•tt•)•+•)                                      DelimGroup
        $                                                         PunctuationToken{tk: "$"}
          (•$•LHS•:•tt•)                                          DelimGroup
            $                                                     PunctuationToken{tk: "$"}
                  :                                               PunctuationToken{tk: ":"}
                         +                                        PunctuationToken{tk: "+"}
                                  {•Expr•!•(•$•(•$•LHS•)•+•)•}    MacroRuleDeclaration.transform{dk: "{}"}
                                         !                        PunctuationToken{tk: "!"}
                                           (•$•(•$•LHS•)•+•)      DelimGroup
                                             $                    PunctuationToken{tk: "$"}
                                               (•$•LHS•)          DelimGroup
                                                 $                PunctuationToken{tk: "$"}
                                                         +        PunctuationToken{tk: "+"}                                               */
    { HList ! [ $ ( $ LHS : tt ) * ] + $ ( $ RHS : tt ) + } => { < Expr ! ( HList ! [ $ ( $ LHS ) * ] ) as Add < Expr ! ( $ ( $ RHS ) + ) >> :: Output } ;/*
    {•HList•!•[•$•(•$•LHS•:•tt•)•*•]•+•$•(•$•RHS•:•tt•)•+•}•=>•{•<•Expr•!•(•HList•!•[•$•(•$•LHS•)•*•]•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::•Output•}    MacroRuleDeclaration
    {•HList•!•[•$•(•$•LHS•:•tt•)•*•]•+•$•(•$•RHS•:•tt•)•+•}                                                                                                 MacroRuleDeclaration.match{dk: "{}"}
            !                                                                                                                                               PunctuationToken{tk: "!"}
              [•$•(•$•LHS•:•tt•)•*•]                                                                                                                        DelimGroup
                $                                                                                                                                           PunctuationToken{tk: "$"}
                  (•$•LHS•:•tt•)                                                                                                                            DelimGroup
                    $                                                                                                                                       PunctuationToken{tk: "$"}
                          :                                                                                                                                 PunctuationToken{tk: ":"}
                                 *                                                                                                                          PunctuationToken{tk: "*"}
                                     +                                                                                                                      PunctuationToken{tk: "+"}
                                       $                                                                                                                    PunctuationToken{tk: "$"}
                                         (•$•RHS•:•tt•)                                                                                                     DelimGroup
                                           $                                                                                                                PunctuationToken{tk: "$"}
                                                 :                                                                                                          PunctuationToken{tk: ":"}
                                                        +                                                                                                   PunctuationToken{tk: "+"}
                                                               {•<•Expr•!•(•HList•!•[•$•(•$•LHS•)•*•]•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::•Output•}    MacroRuleDeclaration.transform{dk: "{}"}
                                                                 <                                                                                          PunctuationToken{tk: "<"}
                                                                        !                                                                                   PunctuationToken{tk: "!"}
                                                                          (•HList•!•[•$•(•$•LHS•)•*•]•)                                                     DelimGroup
                                                                                  !                                                                         PunctuationToken{tk: "!"}
                                                                                    [•$•(•$•LHS•)•*•]                                                       DelimGroup
                                                                                      $                                                                     PunctuationToken{tk: "$"}
                                                                                        (•$•LHS•)                                                           DelimGroup
                                                                                          $                                                                 PunctuationToken{tk: "$"}
                                                                                                  *                                                         PunctuationToken{tk: "*"}
                                                                                                               <                                            PunctuationToken{tk: "<"}
                                                                                                                      !                                     PunctuationToken{tk: "!"}
                                                                                                                        (•$•(•$•RHS•)•+•)                   DelimGroup
                                                                                                                          $                                 PunctuationToken{tk: "$"}
                                                                                                                            (•$•RHS•)                       DelimGroup
                                                                                                                              $                             PunctuationToken{tk: "$"}
                                                                                                                                      +                     PunctuationToken{tk: "+"}
                                                                                                                                          >>                PunctuationToken{tk: ">>"}
                                                                                                                                             ::             PunctuationToken{tk: "::"}*/
    { $ LHS : tt + $ ( $ RHS : tt ) + } => { < Expr ! ( $ LHS ) as Add < Expr ! ( $ ( $ RHS ) + ) >> ::  Output } ;                       /*
    {•$•LHS•:•tt•+•$•(•$•RHS•:•tt•)•+•}•=>•{•<•Expr•!•(•$•LHS•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::••Output•}    MacroRuleDeclaration
    {•$•LHS•:•tt•+•$•(•$•RHS•:•tt•)•+•}                                                                              MacroRuleDeclaration.match{dk: "{}"}
      $                                                                                                              PunctuationToken{tk: "$"}
            :                                                                                                        PunctuationToken{tk: ":"}
                 +                                                                                                   PunctuationToken{tk: "+"}
                   $                                                                                                 PunctuationToken{tk: "$"}
                     (•$•RHS•:•tt•)                                                                                  DelimGroup
                       $                                                                                             PunctuationToken{tk: "$"}
                             :                                                                                       PunctuationToken{tk: ":"}
                                    +                                                                                PunctuationToken{tk: "+"}
                                           {•<•Expr•!•(•$•LHS•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::••Output•}    MacroRuleDeclaration.transform{dk: "{}"}
                                             <                                                                       PunctuationToken{tk: "<"}
                                                    !                                                                PunctuationToken{tk: "!"}
                                                      (•$•LHS•)                                                      DelimGroup
                                                        $                                                            PunctuationToken{tk: "$"}
                                                                       <                                             PunctuationToken{tk: "<"}
                                                                              !                                      PunctuationToken{tk: "!"}
                                                                                (•$•(•$•RHS•)•+•)                    DelimGroup
                                                                                  $                                  PunctuationToken{tk: "$"}
                                                                                    (•$•RHS•)                        DelimGroup
                                                                                      $                              PunctuationToken{tk: "$"}
                                                                                              +                      PunctuationToken{tk: "+"}
                                                                                                  >>                 PunctuationToken{tk: ">>"}
                                                                                                     ::              PunctuationToken{tk: "::"}*/
    { $ LHS : ty } => { $ LHS } ;                                                                                                         /*
    {•$•LHS•:•ty•}•=>•{•$•LHS•}    MacroRuleDeclaration
    {•$•LHS•:•ty•}                 MacroRuleDeclaration.match{dk: "{}"}
      $                            PunctuationToken{tk: "$"}
            :                      PunctuationToken{tk: ":"}
                      {•$•LHS•}    MacroRuleDeclaration.transform{dk: "{}"}
                        $          PunctuationToken{tk: "$"}                                                                              */
    ($($name:ident {                                                                                                                      /*
    ($($name:ident•{↲    <MacroRuleDeclaration>
    ($($name:ident•{↲    <MacroRuleDeclaration.match{dk: "()"}>
     $($name:ident•{↲    <MacroGroup{kind: "*"}>
      ($name:ident•{↲    <MacroGroup.segments{dk: "()"}>
       $name:ident       MacroParameterDeclaration
       $name             McIdentifier
                   {↲    <DelimGroup>                                                                                                     */
        $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*                                                        /*
        $(fn•$method:ident($($arg:ident:•$arg_ty:ty),*•$(,)?)•$(->•$ret_ty:ty)*;)*    MacroGroup{kind: "*"}
         (fn•$method:ident($($arg:ident:•$arg_ty:ty),*•$(,)?)•$(->•$ret_ty:ty)*;)     MacroGroup.segments{dk: "()"}
             $method:ident                                                            MacroParameterDeclaration
             $method                                                                  McIdentifier
                          ($($arg:ident:•$arg_ty:ty),*•$(,)?)                         DelimGroup
                           $($arg:ident:•$arg_ty:ty),*                                MacroGroup{kind: "*"}
                            ($arg:ident:•$arg_ty:ty)                                  MacroGroup.segments{dk: "()"}
                             $arg:ident                                               MacroParameterDeclaration
                             $arg                                                     McIdentifier
                                       :                                              PunctuationToken{tk: ":"}
                                         $arg_ty:ty                                   MacroParameterDeclaration
                                         $arg_ty                                      McIdentifier
                                                    ,                                 PunctuationToken{tk: ","}
                                                       $(,)?                          MacroGroup{kind: "?"}
                                                        (,)                           MacroGroup.segments{dk: "()"}
                                                         ,                            PunctuationToken{tk: ","}
                                                              $(->•$ret_ty:ty)*       MacroGroup{kind: "*"}
                                                               (->•$ret_ty:ty)        MacroGroup.segments{dk: "()"}
                                                                ->                    PunctuationToken{tk: "->"}
                                                                   $ret_ty:ty         MacroParameterDeclaration
                                                                   $ret_ty            McIdentifier
                                                                               ;      PunctuationToken{tk: ";"}                           */
    }),* $(,)?) => {};                                                                                                                    /*
••••}                    </DelimGroup>
••••})                   </MacroGroup.segments>
      ,                  PunctuationToken{tk: ","}
••••}),*                 </MacroGroup>
         $(,)?           MacroGroup{kind: "?"}
          (,)            MacroGroup.segments{dk: "()"}
           ,             PunctuationToken{tk: ","}
••••}),*•$(,)?)          </MacroRuleDeclaration.match>
                   {}    MacroRuleDeclaration.transform{dk: "{}"}
••••}),*•$(,)?)•=>•{}    </MacroRuleDeclaration>                                                                                          */
}                                                                                                                                         /*
}    </MacroRulesDeclaration.rules>
}    </MacroRulesDeclaration>
}    </Program.ast>
}    </Program>                                                                                                                           */
// Discarded Nodes: 0
// Parsed Nodes: 1950
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 9560 (12% re-reads)
// Unnecessary 'skip_whitespace()' calls: 724
// source: "../../samples/macro/macro.match.rs"