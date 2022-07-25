
pub macro c {                                                                                                                             /*
pub•macro•c•{↲    <MacroDeclaration>
pub               PubSpecifier                                                                                                            */
    () => (),                                                                                                                             /*
    ()•=>•()     MacroRuleDeclaration                                                                                                     */
    // _______
    //•_______    Comment
    // _______
    //•_______    Comment
    ($msg:expr $(,)?) => (),                                                                                                              /*
    ($msg:expr•$(,)?)•=>•()     MacroRuleDeclaration
     $msg:expr                  MacroParameterDeclaration
     $msg                       McIdentifier
               $(,)?            MacroGroup
                 ,              PunctuationToken                                                                                          */
    ($fmt:expr, $($arg:tt)*) => (),                                                                                                       /*
    ($fmt:expr,•$($arg:tt)*)•=>•()     MacroRuleDeclaration
     $fmt:expr                         MacroParameterDeclaration
     $fmt                              McIdentifier
              ,                        PunctuationToken
                $($arg:tt)*            MacroGroup
                  $arg:tt              MacroParameterDeclaration
                  $arg                 McIdentifier                                                                                       */
    ($foo:expr) => {};                                                                                                                    /*
    ($foo:expr)•=>•{}     MacroRuleDeclaration
     $foo:expr            MacroParameterDeclaration
     $foo                 McIdentifier                                                                                                    */
    ($($t:tt)+) => (),                                                                                                                    /*
    ($($t:tt)+)•=>•()     MacroRuleDeclaration
     $($t:tt)+            MacroGroup
       $t:tt              MacroParameterDeclaration
       $t                 McIdentifier                                                                                                    */
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => (),                                                      /*
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?•$(,)?)•=>•()     MacroRuleDeclaration
     $left:expr                                                                         MacroParameterDeclaration
     $left                                                                              McIdentifier
               ,                                                                        PunctuationToken
                 $(|)?                                                                  MacroGroup
                   |                                                                    PunctuationToken
                       $(•$pattern:pat_param•)|+                                        MacroGroup
                          $pattern:pat_param                                            MacroParameterDeclaration
                          $pattern                                                      McIdentifier
                                              |                                         PunctuationToken
                                                 $(•if•$guard:•expr•)?                  MacroGroup
                                                       $guard:•expr                     MacroParameterDeclaration
                                                       $guard                           McIdentifier
                                                                       $(,)?            MacroGroup
                                                                         ,              PunctuationToken                                  */
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $($arg:tt)+) => (),                                               /*
    ($left:expr,•$(|)?•$(•$pattern:pat_param•)|+•$(•if•$guard:•expr•)?,•$($arg:tt)+)•=>•()     MacroRuleDeclaration
     $left:expr                                                                                MacroParameterDeclaration
     $left                                                                                     McIdentifier
               ,                                                                               PunctuationToken
                 $(|)?                                                                         MacroGroup
                   |                                                                           PunctuationToken
                       $(•$pattern:pat_param•)|+                                               MacroGroup
                          $pattern:pat_param                                                   MacroParameterDeclaration
                          $pattern                                                             McIdentifier
                                              |                                                PunctuationToken
                                                 $(•if•$guard:•expr•)?                         MacroGroup
                                                       $guard:•expr                            MacroParameterDeclaration
                                                       $guard                                  McIdentifier
                                                                      ,                        PunctuationToken
                                                                        $($arg:tt)+            MacroGroup
                                                                          $arg:tt              MacroParameterDeclaration
                                                                          $arg                 McIdentifier                               */
    // ______
    //•______    Comment
    ("{}", $aze:expr $(,)?) => (),                                                                                                        /*
    ("{}",•$aze:expr•$(,)?)•=>•()     MacroRuleDeclaration
     "{}"                             Literal
         ,                            PunctuationToken
           $aze:expr                  MacroParameterDeclaration
           $aze                       McIdentifier
                     $(,)?            MacroGroup
                       ,              PunctuationToken                                                                                    */
    ($($t:tt)+) => (),                                                                                                                    /*
    ($($t:tt)+)•=>•()     MacroRuleDeclaration
     $($t:tt)+            MacroGroup
       $t:tt              MacroParameterDeclaration
       $t                 McIdentifier                                                                                                    */
    ($x:pat | $y:pat) => {}                                                                                                               /*
    ($x:pat•|•$y:pat)•=>•{}    MacroRuleDeclaration
     $x:pat                    MacroParameterDeclaration
     $x                        McIdentifier
            |                  PunctuationToken
              $y:pat           MacroParameterDeclaration
              $y               McIdentifier                                                                                               */
    ($($x:pat)+ | $($y:pat)+) => {}                                                                                                       /*
    ($($x:pat)+•|•$($y:pat)+)•=>•{}    MacroRuleDeclaration
     $($x:pat)+                        MacroGroup
       $x:pat                          MacroParameterDeclaration
       $x                              McIdentifier
                |                      PunctuationToken
                  $($y:pat)+           MacroGroup
                    $y:pat             MacroParameterDeclaration
                    $y                 McIdentifier                                                                                       */
    ($x:pat_param | $y:pat_param) => {}                                                                                                   /*
    ($x:pat_param•|•$y:pat_param)•=>•{}    MacroRuleDeclaration
     $x:pat_param                          MacroParameterDeclaration
     $x                                    McIdentifier
                  |                        PunctuationToken
                    $y:pat_param           MacroParameterDeclaration
                    $y                     McIdentifier                                                                                   */
    ($x:pat_param | $y:pat) => {}                                                                                                         /*
    ($x:pat_param•|•$y:pat)•=>•{}    MacroRuleDeclaration
     $x:pat_param                    MacroParameterDeclaration
     $x                              McIdentifier
                  |                  PunctuationToken
                    $y:pat           MacroParameterDeclaration
                    $y               McIdentifier                                                                                         */
    ($x:pat | $y:pat_param) => {}                                                                                                         /*
    ($x:pat•|•$y:pat_param)•=>•{}    MacroRuleDeclaration
     $x:pat                          MacroParameterDeclaration
     $x                              McIdentifier
            |                        PunctuationToken
              $y:pat_param           MacroParameterDeclaration
              $y                     McIdentifier                                                                                         */
}                                                                                                                                         /*
}    </MacroDeclaration>                                                                                                                  */

macro_rules! c {                                                                                                                          /*
macro_rules!•c•{↲    <MacroRulesDeclaration>                                                                                              */
    // ____
    //•____    Comment
    // ____
    //•____    Comment
    // ____
    //•____    Comment
    //
    //    Comment
    // ____
    //•____    Comment
    // ____
    //•____    Comment
    // ____
    //•____    Comment
    // ____
    //•____    Comment
    //
    //    Comment
    // ____
    //•____    Comment
    // ____
    //•____    Comment

    // ____
    //•____    Comment
    (@ {                                                                                                                                  /*
    (@•{↲    <MacroRuleDeclaration>
     @       PunctuationToken
       {↲    <DelimGroup>                                                                                                                 */
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        start=$start:expr;                                                                                                                /*
             =                PunctuationToken
              $start:expr     MacroParameterDeclaration
              $start          McIdentifier
                         ;    PunctuationToken                                                                                            */

        // ____
        //•____    Comment
        // ____
        //•____    Comment
        ( $($count:tt)* )                                                                                                                 /*
        (•$($count:tt)*•)    DelimGroup
          $($count:tt)*      MacroGroup
            $count:tt        MacroParameterDeclaration
            $count           McIdentifier                                                                                                 */

        // ____
        //•____    Comment
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        $( ( $($skip:tt)* ) $bind:pat = $fut:expr, if $c:expr => $handle:expr, )+                                                         /*
        $(•(•$($skip:tt)*•)•$bind:pat•=•$fut:expr,•if•$c:expr•=>•$handle:expr,•)+    MacroGroup
           (•$($skip:tt)*•)                                                          DelimGroup
             $($skip:tt)*                                                            MacroGroup
               $skip:tt                                                              MacroParameterDeclaration
               $skip                                                                 McIdentifier
                            $bind:pat                                                MacroParameterDeclaration
                            $bind                                                    McIdentifier
                                      =                                              PunctuationToken
                                        $fut:expr                                    MacroParameterDeclaration
                                        $fut                                         McIdentifier
                                                 ,                                   PunctuationToken
                                                      $c:expr                        MacroParameterDeclaration
                                                      $c                             McIdentifier
                                                              =>                     PunctuationToken
                                                                 $handle:expr        MacroParameterDeclaration
                                                                 $handle             McIdentifier
                                                                             ,       PunctuationToken                                     */

        // ____
        //•____    Comment
        ; $else:expr                                                                                                                      /*
        ;               PunctuationToken
          $else:expr    MacroParameterDeclaration
          $else         McIdentifier                                                                                                      */

    }) => {};                                                                                                                             /*
••••})•=>•{}     </MacroRuleDeclaration>
••••}            </DelimGroup>                                                                                                            */

    // ____
    //•____    Comment

    // ____
    //•____    Comment
    // ____
    //•____    Comment

    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr, if $c:expr => $h:block, $($r:tt)* ) => {};                        /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:block,•$($r:tt)*•)•=>•{}     MacroRuleDeclaration
     @                                                                                                                PunctuationToken
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                                 DelimGroup
              =                                                                                                       PunctuationToken
               $start:expr                                                                                            MacroParameterDeclaration
               $start                                                                                                 McIdentifier
                          ;                                                                                           PunctuationToken
                            (•$($s:tt)*•)                                                                             DelimGroup
                              $($s:tt)*                                                                               MacroGroup
                                $s:tt                                                                                 MacroParameterDeclaration
                                $s                                                                                    McIdentifier
                                          $($t:tt)*                                                                   MacroGroup
                                            $t:tt                                                                     MacroParameterDeclaration
                                            $t                                                                        McIdentifier
                                                      $p:pat                                                          MacroParameterDeclaration
                                                      $p                                                              McIdentifier
                                                             =                                                        PunctuationToken
                                                               $f:expr                                                MacroParameterDeclaration
                                                               $f                                                     McIdentifier
                                                                      ,                                               PunctuationToken
                                                                           $c:expr                                    MacroParameterDeclaration
                                                                           $c                                         McIdentifier
                                                                                   =>                                 PunctuationToken
                                                                                      $h:block                        MacroParameterDeclaration
                                                                                      $h                              McIdentifier
                                                                                              ,                       PunctuationToken
                                                                                                $($r:tt)*             MacroGroup
                                                                                                  $r:tt               MacroParameterDeclaration
                                                                                                  $r                  McIdentifier        */
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr, if $c:expr => $h:expr ) => {};                                    /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr,•if•$c:expr•=>•$h:expr•)•=>•{}     MacroRuleDeclaration
     @                                                                                                    PunctuationToken
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                     DelimGroup
              =                                                                                           PunctuationToken
               $start:expr                                                                                MacroParameterDeclaration
               $start                                                                                     McIdentifier
                          ;                                                                               PunctuationToken
                            (•$($s:tt)*•)                                                                 DelimGroup
                              $($s:tt)*                                                                   MacroGroup
                                $s:tt                                                                     MacroParameterDeclaration
                                $s                                                                        McIdentifier
                                          $($t:tt)*                                                       MacroGroup
                                            $t:tt                                                         MacroParameterDeclaration
                                            $t                                                            McIdentifier
                                                      $p:pat                                              MacroParameterDeclaration
                                                      $p                                                  McIdentifier
                                                             =                                            PunctuationToken
                                                               $f:expr                                    MacroParameterDeclaration
                                                               $f                                         McIdentifier
                                                                      ,                                   PunctuationToken
                                                                           $c:expr                        MacroParameterDeclaration
                                                                           $c                             McIdentifier
                                                                                   =>                     PunctuationToken
                                                                                      $h:expr             MacroParameterDeclaration
                                                                                      $h                  McIdentifier                    */
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr => $h:expr, $($r:tt)* ) => {};                                     /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr,•$($r:tt)*•)•=>•{}     MacroRuleDeclaration
     @                                                                                                   PunctuationToken
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                                    DelimGroup
              =                                                                                          PunctuationToken
               $start:expr                                                                               MacroParameterDeclaration
               $start                                                                                    McIdentifier
                          ;                                                                              PunctuationToken
                            (•$($s:tt)*•)                                                                DelimGroup
                              $($s:tt)*                                                                  MacroGroup
                                $s:tt                                                                    MacroParameterDeclaration
                                $s                                                                       McIdentifier
                                          $($t:tt)*                                                      MacroGroup
                                            $t:tt                                                        MacroParameterDeclaration
                                            $t                                                           McIdentifier
                                                      $p:pat                                             MacroParameterDeclaration
                                                      $p                                                 McIdentifier
                                                             =                                           PunctuationToken
                                                               $f:expr                                   MacroParameterDeclaration
                                                               $f                                        McIdentifier
                                                                       =>                                PunctuationToken
                                                                          $h:expr                        MacroParameterDeclaration
                                                                          $h                             McIdentifier
                                                                                 ,                       PunctuationToken
                                                                                   $($r:tt)*             MacroGroup
                                                                                     $r:tt               MacroParameterDeclaration
                                                                                     $r                  McIdentifier                     */
    (@ { start=$start:expr; ( $($s:tt)* ) $($t:tt)* } $p:pat = $f:expr => $h:expr ) => {};                                                /*
    (@•{•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}•$p:pat•=•$f:expr•=>•$h:expr•)•=>•{}     MacroRuleDeclaration
     @                                                                                        PunctuationToken
       {•start=$start:expr;•(•$($s:tt)*•)•$($t:tt)*•}                                         DelimGroup
              =                                                                               PunctuationToken
               $start:expr                                                                    MacroParameterDeclaration
               $start                                                                         McIdentifier
                          ;                                                                   PunctuationToken
                            (•$($s:tt)*•)                                                     DelimGroup
                              $($s:tt)*                                                       MacroGroup
                                $s:tt                                                         MacroParameterDeclaration
                                $s                                                            McIdentifier
                                          $($t:tt)*                                           MacroGroup
                                            $t:tt                                             MacroParameterDeclaration
                                            $t                                                McIdentifier
                                                      $p:pat                                  MacroParameterDeclaration
                                                      $p                                      McIdentifier
                                                             =                                PunctuationToken
                                                               $f:expr                        MacroParameterDeclaration
                                                               $f                             McIdentifier
                                                                       =>                     PunctuationToken
                                                                          $h:expr             MacroParameterDeclaration
                                                                          $h                  McIdentifier                                */
    (@ { start=$start:expr; $($t:tt)* } else => $else:expr $(,)?) => {};                                                                  /*
    (@•{•start=$start:expr;•$($t:tt)*•}•else•=>•$else:expr•$(,)?)•=>•{}     MacroRuleDeclaration
     @                                                                      PunctuationToken
       {•start=$start:expr;•$($t:tt)*•}                                     DelimGroup
              =                                                             PunctuationToken
               $start:expr                                                  MacroParameterDeclaration
               $start                                                       McIdentifier
                          ;                                                 PunctuationToken
                            $($t:tt)*                                       MacroGroup
                              $t:tt                                         MacroParameterDeclaration
                              $t                                            McIdentifier
                                             =>                             PunctuationToken
                                                $else:expr                  MacroParameterDeclaration
                                                $else                       McIdentifier
                                                           $(,)?            MacroGroup
                                                             ,              PunctuationToken                                              */
    (@ { start=$start:expr; $($t:tt)* } ) => {};                                                                                          /*
    (@•{•start=$start:expr;•$($t:tt)*•}•)•=>•{}     MacroRuleDeclaration
     @                                              PunctuationToken
       {•start=$start:expr;•$($t:tt)*•}             DelimGroup
              =                                     PunctuationToken
               $start:expr                          MacroParameterDeclaration
               $start                               McIdentifier
                          ;                         PunctuationToken
                            $($t:tt)*               MacroGroup
                              $t:tt                 MacroParameterDeclaration
                              $t                    McIdentifier                                                                          */

    ( ($($id:ident),*) ) => (());                                                                                                         /*
    (•($($id:ident),*)•)•=>•(())     MacroRuleDeclaration
      ($($id:ident),*)               DelimGroup
       $($id:ident),*                MacroGroup
         $id:ident                   MacroParameterDeclaration
         $id                         McIdentifier
                   ,                 PunctuationToken
                             ()      DelimGroup                                                                                           */
    ( [$($id:ident),*] ) => (());                                                                                                         /*
    (•[$($id:ident),*]•)•=>•(())     MacroRuleDeclaration
      [$($id:ident),*]               DelimGroup
       $($id:ident),*                MacroGroup
         $id:ident                   MacroParameterDeclaration
         $id                         McIdentifier
                   ,                 PunctuationToken
                             ()      DelimGroup                                                                                           */
    ( {$($id:ident),*} ) => (());                                                                                                         /*
    (•{$($id:ident),*}•)•=>•(())     MacroRuleDeclaration
      {$($id:ident),*}               DelimGroup
       $($id:ident),*                MacroGroup
         $id:ident                   MacroParameterDeclaration
         $id                         McIdentifier
                   ,                 PunctuationToken
                             ()      DelimGroup                                                                                           */

    // ____
    //•____    Comment

    (biased; $p:pat = $($t:tt)* ) => {};                                                                                                  /*
    (biased;•$p:pat•=•$($t:tt)*•)•=>•{}     MacroRuleDeclaration
           ;                                PunctuationToken
             $p:pat                         MacroParameterDeclaration
             $p                             McIdentifier
                    =                       PunctuationToken
                      $($t:tt)*             MacroGroup
                        $t:tt               MacroParameterDeclaration
                        $t                  McIdentifier                                                                                  */

    ( $p:pat = $($t:tt)* ) => {};                                                                                                         /*
    (•$p:pat•=•$($t:tt)*•)•=>•{}     MacroRuleDeclaration
      $p:pat                         MacroParameterDeclaration
      $p                             McIdentifier
             =                       PunctuationToken
               $($t:tt)*             MacroGroup
                 $t:tt               MacroParameterDeclaration
                 $t                  McIdentifier                                                                                         */
    () => {};                                                                                                                             /*
    ()•=>•{}     MacroRuleDeclaration                                                                                                     */
    ($($tts:tt)+) => { loom::thread_local!{ $($tts)+ } };                                                                                 /*
    ($($tts:tt)+)•=>•{•loom::thread_local!{•$($tts)+•}•}     MacroRuleDeclaration
     $($tts:tt)+                                             MacroGroup
       $tts:tt                                               MacroParameterDeclaration
       $tts                                                  McIdentifier
                           ::                                PunctuationToken
                                         !                   PunctuationToken
                                          {•$($tts)+•}       DelimGroup
                                            $($tts)+         MacroGroup
                                              $tts           McIdentifier                                                                 */
    (@ {                                                                                                                                  /*
    (@•{↲    <MacroRuleDeclaration>
     @       PunctuationToken
       {↲    <DelimGroup>                                                                                                                 */
        // ____
        //•____    Comment
        // ____
        //•____    Comment
        ( $($count:tt)* )                                                                                                                 /*
        (•$($count:tt)*•)    DelimGroup
          $($count:tt)*      MacroGroup
            $count:tt        MacroParameterDeclaration
            $count           McIdentifier                                                                                                 */

        // ____
        //•____    Comment
        $( ( $($skip:tt)* ) $e:expr, )*                                                                                                   /*
        $(•(•$($skip:tt)*•)•$e:expr,•)*    MacroGroup
           (•$($skip:tt)*•)                DelimGroup
             $($skip:tt)*                  MacroGroup
               $skip:tt                    MacroParameterDeclaration
               $skip                       McIdentifier
                            $e:expr        MacroParameterDeclaration
                            $e             McIdentifier
                                   ,       PunctuationToken                                                                               */

    }) => {};                                                                                                                             /*
••••})•=>•{}     </MacroRuleDeclaration>
••••}            </DelimGroup>                                                                                                            */

    // ____
    //•____    Comment

    (@ { ( $($s:tt)* ) $($t:tt)* } $e:expr, $($r:tt)* ) => {};                                                                            /*
    (@•{•(•$($s:tt)*•)•$($t:tt)*•}•$e:expr,•$($r:tt)*•)•=>•{}     MacroRuleDeclaration
     @                                                            PunctuationToken
       {•(•$($s:tt)*•)•$($t:tt)*•}                                DelimGroup
         (•$($s:tt)*•)                                            DelimGroup
           $($s:tt)*                                              MacroGroup
             $s:tt                                                MacroParameterDeclaration
             $s                                                   McIdentifier
                       $($t:tt)*                                  MacroGroup
                         $t:tt                                    MacroParameterDeclaration
                         $t                                       McIdentifier
                                   $e:expr                        MacroParameterDeclaration
                                   $e                             McIdentifier
                                          ,                       PunctuationToken
                                            $($r:tt)*             MacroGroup
                                              $r:tt               MacroParameterDeclaration
                                              $r                  McIdentifier                                                            */
    ($($(#[$cfg:meta])* $name:ident,)*) => {};                                                                                            /*
    ($($(#[$cfg:meta])*•$name:ident,)*)•=>•{}     MacroRuleDeclaration
     $($(#[$cfg:meta])*•$name:ident,)*            MacroGroup
       $(#[$cfg:meta])*                           MacroGroup
         #                                        PunctuationToken
          [$cfg:meta]                             DelimGroup
           $cfg:meta                              MacroParameterDeclaration
           $cfg                                   McIdentifier
                        $name:ident               MacroParameterDeclaration
                        $name                     McIdentifier
                                   ,              PunctuationToken                                                                        */

    // ____
    //•____    Comment

    ( $($e:expr),* $(,)?) => {};                                                                                                          /*
    (•$($e:expr),*•$(,)?)•=>•{}     MacroRuleDeclaration
      $($e:expr),*                  MacroGroup
        $e:expr                     MacroParameterDeclaration
        $e                          McIdentifier
                ,                   PunctuationToken
                   $(,)?            MacroGroup
                     ,              PunctuationToken                                                                                      */
    () => {};                                                                                                                             /*
    ()•=>•{}     MacroRuleDeclaration                                                                                                     */

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => {};                                                           /*
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty;•$($rest:tt)*)•=>•{}     MacroRuleDeclaration
     $(#[$attr:meta])*                                                             MacroGroup
       #                                                                           PunctuationToken
        [$attr:meta]                                                               DelimGroup
         $attr:meta                                                                MacroParameterDeclaration
         $attr                                                                     McIdentifier
                       $vis:vis                                                    MacroParameterDeclaration
                       $vis                                                        McIdentifier
                                       $name:ident                                 MacroParameterDeclaration
                                       $name                                       McIdentifier
                                                  :                                PunctuationToken
                                                    $t:ty                          MacroParameterDeclaration
                                                    $t                             McIdentifier
                                                         ;                         PunctuationToken
                                                           $($rest:tt)*            MacroGroup
                                                             $rest:tt              MacroParameterDeclaration
                                                             $rest                 McIdentifier                                           */

    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty) => {}                                                                          /*
    ($(#[$attr:meta])*•$vis:vis•static•$name:ident:•$t:ty)•=>•{}    MacroRuleDeclaration
     $(#[$attr:meta])*                                              MacroGroup
       #                                                            PunctuationToken
        [$attr:meta]                                                DelimGroup
         $attr:meta                                                 MacroParameterDeclaration
         $attr                                                      McIdentifier
                       $vis:vis                                     MacroParameterDeclaration
                       $vis                                         McIdentifier
                                       $name:ident                  MacroParameterDeclaration
                                       $name                        McIdentifier
                                                  :                 PunctuationToken
                                                    $t:ty           MacroParameterDeclaration
                                                    $t              McIdentifier                                                          */
    (Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};                                                                                   /*
    (Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)•=>•{}     MacroRuleDeclaration
          &                                                PunctuationToken
            $(!)?                                          MacroGroup
              !                                            PunctuationToken
                      &                                    PunctuationToken
                        $(!)?                              MacroGroup
                          !                                PunctuationToken
                                  ,                        PunctuationToken
                                    $value:expr            MacroParameterDeclaration
                                    $value                 McIdentifier                                                                   */
    (!Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};                                                                                  /*
    (!Send•&•$(!)?Sync•&•$(!)?Unpin,•$value:expr)•=>•{}     MacroRuleDeclaration
     !                                                      PunctuationToken
           &                                                PunctuationToken
             $(!)?                                          MacroGroup
               !                                            PunctuationToken
                       &                                    PunctuationToken
                         $(!)?                              MacroGroup
                           !                                PunctuationToken
                                   ,                        PunctuationToken
                                     $value:expr            MacroParameterDeclaration
                                     $value                 McIdentifier                                                                  */
    ($($f:ident $(< $($generic:ty),* > )? )::+($($arg:ty),*): $($tok:tt)*) => {};                                                         /*
    ($($f:ident•$(<•$($generic:ty),*•>•)?•)::+($($arg:ty),*):•$($tok:tt)*)•=>•{}     MacroRuleDeclaration
     $($f:ident•$(<•$($generic:ty),*•>•)?•)::+                                       MacroGroup
       $f:ident                                                                      MacroParameterDeclaration
       $f                                                                            McIdentifier
                $(<•$($generic:ty),*•>•)?                                            MacroGroup
                  <                                                                  PunctuationToken
                    $($generic:ty),*                                                 MacroGroup
                      $generic:ty                                                    MacroParameterDeclaration
                      $generic                                                       McIdentifier
                                  ,                                                  PunctuationToken
                                     >                                               PunctuationToken
                                           ::                                        PunctuationToken
                                              ($($arg:ty),*)                         DelimGroup
                                               $($arg:ty),*                          MacroGroup
                                                 $arg:ty                             MacroParameterDeclaration
                                                 $arg                                McIdentifier
                                                         ,                           PunctuationToken
                                                            :                        PunctuationToken
                                                              $($tok:tt)*            MacroGroup
                                                                $tok:tt              MacroParameterDeclaration
                                                                $tok                 McIdentifier                                         */
    ($i:ident, $start:ident, $($delta:expr),*$(,)?) => {};                                                                                /*
    ($i:ident,•$start:ident,•$($delta:expr),*$(,)?)•=>•{}     MacroRuleDeclaration
     $i:ident                                                 MacroParameterDeclaration
     $i                                                       McIdentifier
             ,                                                PunctuationToken
               $start:ident                                   MacroParameterDeclaration
               $start                                         McIdentifier
                           ,                                  PunctuationToken
                             $($delta:expr),*                 MacroGroup
                               $delta:expr                    MacroParameterDeclaration
                               $delta                         McIdentifier
                                           ,                  PunctuationToken
                                             $(,)?            MacroGroup
                                               ,              PunctuationToken                                                            */
    ($i:ident, $start:ident) => {};                                                                                                       /*
    ($i:ident,•$start:ident)•=>•{}     MacroRuleDeclaration
     $i:ident                          MacroParameterDeclaration
     $i                                McIdentifier
             ,                         PunctuationToken
               $start:ident            MacroParameterDeclaration
               $start                  McIdentifier                                                                                       */
    ($($name:ident = $sem:ident($bits:tt : $exp_bits:tt)),*) => {};                                                                       /*
    ($($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt)),*)•=>•{}     MacroRuleDeclaration
     $($name:ident•=•$sem:ident($bits:tt•:•$exp_bits:tt)),*            MacroGroup
       $name:ident                                                     MacroParameterDeclaration
       $name                                                           McIdentifier
                   =                                                   PunctuationToken
                     $sem:ident                                        MacroParameterDeclaration
                     $sem                                              McIdentifier
                               ($bits:tt•:•$exp_bits:tt)               DelimGroup
                                $bits:tt                               MacroParameterDeclaration
                                $bits                                  McIdentifier
                                         :                             PunctuationToken
                                           $exp_bits:tt                MacroParameterDeclaration
                                           $exp_bits                   McIdentifier
                                                         ,             PunctuationToken                                                   */
    ($ty:ident<$t:tt>) => {};                                                                                                             /*
    ($ty:ident<$t:tt>)•=>•{}     MacroRuleDeclaration
     $ty:ident                   MacroParameterDeclaration
     $ty                         McIdentifier
              <                  PunctuationToken
               $t:tt             MacroParameterDeclaration
               $t                McIdentifier
                    >            PunctuationToken                                                                                         */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        const SUPPORTS_CUSTOM_INNER_ATTRS: bool = $inner_attrs:literal;                                                                   /*
                                         :                                 PunctuationToken
                                                =                          PunctuationToken
                                                  $inner_attrs:literal     MacroParameterDeclaration
                                                  $inner_attrs             McIdentifier
                                                                      ;    PunctuationToken                                               */
        $($ty:path),*                                                                                                                     /*
        $($ty:path),*    MacroGroup
          $ty:path       MacroParameterDeclaration
          $ty            McIdentifier
                   ,     PunctuationToken                                                                                                 */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    ($($name:literal => $feature:ident)*) => {};                                                                                          /*
    ($($name:literal•=>•$feature:ident)*)•=>•{}     MacroRuleDeclaration
     $($name:literal•=>•$feature:ident)*            MacroGroup
       $name:literal                                MacroParameterDeclaration
       $name                                        McIdentifier
                     =>                             PunctuationToken
                        $feature:ident              MacroParameterDeclaration
                        $feature                    McIdentifier                                                                          */
    ( $(                                                                                                                                  /*
    (•$(↲    <MacroRuleDeclaration>
      $(↲    <MacroGroup>                                                                                                                 */
        $T:ident { $( $field:ident : $A:ident ),* $(,)? }                                                                                 /*
        $T:ident                                             MacroParameterDeclaration
        $T                                                   McIdentifier
                 {•$(•$field:ident•:•$A:ident•),*•$(,)?•}    DelimGroup
                   $(•$field:ident•:•$A:ident•),*            MacroGroup
                      $field:ident                           MacroParameterDeclaration
                      $field                                 McIdentifier
                                   :                         PunctuationToken
                                     $A:ident                MacroParameterDeclaration
                                     $A                      McIdentifier
                                               ,             PunctuationToken
                                                  $(,)?      MacroGroup
                                                    ,        PunctuationToken                                                             */
    )* ) => {};                                                                                                                           /*
••••)*•)•=>•{}     </MacroRuleDeclaration>
••••)*             </MacroGroup>                                                                                                          */
    ($wr:ident . write_facts_to_path($this:ident . [                                                                                      /*
    ($wr:ident•.•write_facts_to_path($this:ident•.•[↲    <MacroRuleDeclaration>
     $wr:ident                                           MacroParameterDeclaration
     $wr                                                 McIdentifier
               .                                         PunctuationToken
                                    ($this:ident•.•[↲    <DelimGroup>
                                     $this:ident         MacroParameterDeclaration
                                     $this               McIdentifier
                                                 .       PunctuationToken
                                                   [↲    <DelimGroup>                                                                     */
        $($field:ident,)*                                                                                                                 /*
        $($field:ident,)*    MacroGroup
          $field:ident       MacroParameterDeclaration
          $field             McIdentifier
                      ,      PunctuationToken                                                                                             */
    ])) => {};                                                                                                                            /*
••••]))•=>•{}     </MacroRuleDeclaration>
••••])            </DelimGroup>
••••]             </DelimGroup>                                                                                                           */
    ($in_:expr, {                                                                                                                         /*
    ($in_:expr,•{↲    <MacroRuleDeclaration>
     $in_:expr        MacroParameterDeclaration
     $in_             McIdentifier
              ,       PunctuationToken
                {↲    <DelimGroup>                                                                                                        */
        $param:expr, $flags:expr,                                                                                                         /*
        $param:expr                  MacroParameterDeclaration
        $param                       McIdentifier
                   ,                 PunctuationToken
                     $flags:expr     MacroParameterDeclaration
                     $flags          McIdentifier
                                ,    PunctuationToken                                                                                     */
        $width:expr, $prec:expr, $len:expr, $type_:expr,                                                                                  /*
        $width:expr                                         MacroParameterDeclaration
        $width                                              McIdentifier
                   ,                                        PunctuationToken
                     $prec:expr                             MacroParameterDeclaration
                     $prec                                  McIdentifier
                               ,                            PunctuationToken
                                 $len:expr                  MacroParameterDeclaration
                                 $len                       McIdentifier
                                          ,                 PunctuationToken
                                            $type_:expr     MacroParameterDeclaration
                                            $type_          McIdentifier
                                                       ,    PunctuationToken                                                              */
        $pos:expr,                                                                                                                        /*
        $pos:expr     MacroParameterDeclaration
        $pos          McIdentifier
                 ,    PunctuationToken                                                                                                    */
    }) => {};                                                                                                                             /*
••••})•=>•{}     </MacroRuleDeclaration>
••••}            </DelimGroup>                                                                                                            */
    (($($dollar:tt $placeholder:ident)*); $($($values:ident),+);*: $($test:tt)*) => {};                                                   /*
    (($($dollar:tt•$placeholder:ident)*);•$($($values:ident),+);*:•$($test:tt)*)•=>•{}     MacroRuleDeclaration
     ($($dollar:tt•$placeholder:ident)*)                                                   DelimGroup
      $($dollar:tt•$placeholder:ident)*                                                    MacroGroup
        $dollar:tt                                                                         MacroParameterDeclaration
        $dollar                                                                            McIdentifier
                   $placeholder:ident                                                      MacroParameterDeclaration
                   $placeholder                                                            McIdentifier
                                        ;                                                  PunctuationToken
                                          $($($values:ident),+);*                          MacroGroup
                                            $($values:ident),+                             MacroGroup
                                              $values:ident                                MacroParameterDeclaration
                                              $values                                      McIdentifier
                                                            ,                              PunctuationToken
                                                               ;                           PunctuationToken
                                                                 :                         PunctuationToken
                                                                   $($test:tt)*            MacroGroup
                                                                     $test:tt              MacroParameterDeclaration
                                                                     $test                 McIdentifier                                   */
    ($($name: ident: $($($p: ident),* => $call: ident),*;)*) => {};                                                                       /*
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)•=>•{}     MacroRuleDeclaration
     $($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*            MacroGroup
       $name:•ident                                                    MacroParameterDeclaration
       $name                                                           McIdentifier
                   :                                                   PunctuationToken
                     $($($p:•ident),*•=>•$call:•ident),*               MacroGroup
                       $($p:•ident),*                                  MacroGroup
                         $p:•ident                                     MacroParameterDeclaration
                         $p                                            McIdentifier
                                   ,                                   PunctuationToken
                                      =>                               PunctuationToken
                                         $call:•ident                  MacroParameterDeclaration
                                         $call                         McIdentifier
                                                      ,                PunctuationToken
                                                        ;              PunctuationToken                                                   */
    ($($name:ident($($arg:ident),*) => $llvm_capi:ident),+ $(,)?) => {};                                                                  /*
    ($($name:ident($($arg:ident),*)•=>•$llvm_capi:ident),+•$(,)?)•=>•{}     MacroRuleDeclaration
     $($name:ident($($arg:ident),*)•=>•$llvm_capi:ident),+                  MacroGroup
       $name:ident                                                          MacroParameterDeclaration
       $name                                                                McIdentifier
                  ($($arg:ident),*)                                         DelimGroup
                   $($arg:ident),*                                          MacroGroup
                     $arg:ident                                             MacroParameterDeclaration
                     $arg                                                   McIdentifier
                                ,                                           PunctuationToken
                                    =>                                      PunctuationToken
                                       $llvm_capi:ident                     MacroParameterDeclaration
                                       $llvm_capi                           McIdentifier
                                                        ,                   PunctuationToken
                                                           $(,)?            MacroGroup
                                                             ,              PunctuationToken                                              */
    ($name:expr, fn() -> $ret:expr) => {};                                                                                                /*
    ($name:expr,•fn()•->•$ret:expr)•=>•{}     MacroRuleDeclaration
     $name:expr                               MacroParameterDeclaration
     $name                                    McIdentifier
               ,                              PunctuationToken
                   ()                         DelimGroup
                      ->                      PunctuationToken
                         $ret:expr            MacroParameterDeclaration
                         $ret                 McIdentifier                                                                                */
    ($name:expr, fn(...) -> $ret:expr) => {};                                                                                             /*
    ($name:expr,•fn(...)•->•$ret:expr)•=>•{}     MacroRuleDeclaration
     $name:expr                                  MacroParameterDeclaration
     $name                                       McIdentifier
               ,                                 PunctuationToken
                   (...)                         DelimGroup
                    ...                          PunctuationToken
                         ->                      PunctuationToken
                            $ret:expr            MacroParameterDeclaration
                            $ret                 McIdentifier                                                                             */
    ($name:expr, fn($($arg:expr),*) -> $ret:expr) => {};                                                                                  /*
    ($name:expr,•fn($($arg:expr),*)•->•$ret:expr)•=>•{}     MacroRuleDeclaration
     $name:expr                                             MacroParameterDeclaration
     $name                                                  McIdentifier
               ,                                            PunctuationToken
                   ($($arg:expr),*)                         DelimGroup
                    $($arg:expr),*                          MacroGroup
                      $arg:expr                             MacroParameterDeclaration
                      $arg                                  McIdentifier
                                ,                           PunctuationToken
                                    ->                      PunctuationToken
                                       $ret:expr            MacroParameterDeclaration
                                       $ret                 McIdentifier                                                                  */
    ($($field_ty:expr),*) => {};                                                                                                          /*
    ($($field_ty:expr),*)•=>•{}     MacroRuleDeclaration
     $($field_ty:expr),*            MacroGroup
       $field_ty:expr               MacroParameterDeclaration
       $field_ty                    McIdentifier
                      ,             PunctuationToken                                                                                      */
    ($($name: ident: $($($p: ident),* => $call: ident),*;)*) => {};                                                                       /*
    ($($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*)•=>•{}     MacroRuleDeclaration
     $($name:•ident:•$($($p:•ident),*•=>•$call:•ident),*;)*            MacroGroup
       $name:•ident                                                    MacroParameterDeclaration
       $name                                                           McIdentifier
                   :                                                   PunctuationToken
                     $($($p:•ident),*•=>•$call:•ident),*               MacroGroup
                       $($p:•ident),*                                  MacroGroup
                         $p:•ident                                     MacroParameterDeclaration
                         $p                                            McIdentifier
                                   ,                                   PunctuationToken
                                      =>                               PunctuationToken
                                         $call:•ident                  MacroParameterDeclaration
                                         $call                         McIdentifier
                                                      ,                PunctuationToken
                                                        ;              PunctuationToken                                                   */
    ($where:expr, { $( $what_fmt:expr ),+ } $( expected { $( $expected_fmt:expr ),+ } )?) => {};                                          /*
    ($where:expr,•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?)•=>•{}     MacroRuleDeclaration
     $where:expr                                                                                    MacroParameterDeclaration
     $where                                                                                         McIdentifier
                ,                                                                                   PunctuationToken
                  {•$(•$what_fmt:expr•),+•}                                                         DelimGroup
                    $(•$what_fmt:expr•),+                                                           MacroGroup
                       $what_fmt:expr                                                               MacroParameterDeclaration
                       $what_fmt                                                                    McIdentifier
                                       ,                                                            PunctuationToken
                                            $(•expected•{•$(•$expected_fmt:expr•),+•}•)?            MacroGroup
                                                        {•$(•$expected_fmt:expr•),+•}               DelimGroup
                                                          $(•$expected_fmt:expr•),+                 MacroGroup
                                                             $expected_fmt:expr                     MacroParameterDeclaration
                                                             $expected_fmt                          McIdentifier
                                                                                 ,                  PunctuationToken                      */
    ($e:expr, $where:expr,                                                                                                                /*
    ($e:expr,•$where:expr,↲    <MacroRuleDeclaration>
     $e:expr                   MacroParameterDeclaration
     $e                        McIdentifier
            ,                  PunctuationToken
              $where:expr      MacroParameterDeclaration
              $where           McIdentifier
                         ,     PunctuationToken                                                                                           */
    $( $( $p:pat_param )|+ => { $( $what_fmt:expr ),+ } $( expected { $( $expected_fmt:expr ),+ } )? ),+ $(,)?                            /*
    $(•$(•$p:pat_param•)|+•=>•{•$(•$what_fmt:expr•),+•}•$(•expected•{•$(•$expected_fmt:expr•),+•}•)?•),+          MacroGroup
       $(•$p:pat_param•)|+                                                                                        MacroGroup
          $p:pat_param                                                                                            MacroParameterDeclaration
          $p                                                                                                      McIdentifier
                        |                                                                                         PunctuationToken
                           =>                                                                                     PunctuationToken
                              {•$(•$what_fmt:expr•),+•}                                                           DelimGroup
                                $(•$what_fmt:expr•),+                                                             MacroGroup
                                   $what_fmt:expr                                                                 MacroParameterDeclaration
                                   $what_fmt                                                                      McIdentifier
                                                   ,                                                              PunctuationToken
                                                        $(•expected•{•$(•$expected_fmt:expr•),+•}•)?              MacroGroup
                                                                    {•$(•$expected_fmt:expr•),+•}                 DelimGroup
                                                                      $(•$expected_fmt:expr•),+                   MacroGroup
                                                                         $expected_fmt:expr                       MacroParameterDeclaration
                                                                         $expected_fmt                            McIdentifier
                                                                                             ,                    PunctuationToken
                                                                                                      ,           PunctuationToken
                                                                                                         $(,)?    MacroGroup
                                                                                                           ,      PunctuationToken        */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    ($(#[$attr:meta])* pub enum $name:ident {                                                                                             /*
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration>
     $(#[$attr:meta])*                            MacroGroup
       #                                          PunctuationToken
        [$attr:meta]                              DelimGroup
         $attr:meta                               MacroParameterDeclaration
         $attr                                    McIdentifier
                                $name:ident       MacroParameterDeclaration
                                $name             McIdentifier
                                            {↲    <DelimGroup>                                                                            */
        $($(#[$var_attr:meta])* $variant:ident = $e:expr,)*                                                                               /*
        $($(#[$var_attr:meta])*•$variant:ident•=•$e:expr,)*    MacroGroup
          $(#[$var_attr:meta])*                                MacroGroup
            #                                                  PunctuationToken
             [$var_attr:meta]                                  DelimGroup
              $var_attr:meta                                   MacroParameterDeclaration
              $var_attr                                        McIdentifier
                                $variant:ident                 MacroParameterDeclaration
                                $variant                       McIdentifier
                                               =               PunctuationToken
                                                 $e:expr       MacroParameterDeclaration
                                                 $e            McIdentifier
                                                        ,      PunctuationToken                                                           */
    }) => {};                                                                                                                             /*
••••})•=>•{}     </MacroRuleDeclaration>
••••}            </DelimGroup>                                                                                                            */
    ($(#[$attr:meta])* pub enum $name:ident {                                                                                             /*
    ($(#[$attr:meta])*•pub•enum•$name:ident•{↲    <MacroRuleDeclaration>
     $(#[$attr:meta])*                            MacroGroup
       #                                          PunctuationToken
        [$attr:meta]                              DelimGroup
         $attr:meta                               MacroParameterDeclaration
         $attr                                    McIdentifier
                                $name:ident       MacroParameterDeclaration
                                $name             McIdentifier
                                            {↲    <DelimGroup>                                                                            */
        $($(#[$var_attr:meta])* $variant:ident,)*                                                                                         /*
        $($(#[$var_attr:meta])*•$variant:ident,)*    MacroGroup
          $(#[$var_attr:meta])*                      MacroGroup
            #                                        PunctuationToken
             [$var_attr:meta]                        DelimGroup
              $var_attr:meta                         MacroParameterDeclaration
              $var_attr                              McIdentifier
                                $variant:ident       MacroParameterDeclaration
                                $variant             McIdentifier
                                              ,      PunctuationToken                                                                     */
    }) => {};                                                                                                                             /*
••••})•=>•{}     </MacroRuleDeclaration>
••••}            </DelimGroup>                                                                                                            */
    (impl $fblock:tt [$($c:tt,)*] [$block:tt $(, $rest:tt)*]) => {};                                                                      /*
    (impl•$fblock:tt•[$($c:tt,)*]•[$block:tt•$(,•$rest:tt)*])•=>•{}     MacroRuleDeclaration
          $fblock:tt                                                    MacroParameterDeclaration
          $fblock                                                       McIdentifier
                     [$($c:tt,)*]                                       DelimGroup
                      $($c:tt,)*                                        MacroGroup
                        $c:tt                                           MacroParameterDeclaration
                        $c                                              McIdentifier
                             ,                                          PunctuationToken
                                  [$block:tt•$(,•$rest:tt)*]            DelimGroup
                                   $block:tt                            MacroParameterDeclaration
                                   $block                               McIdentifier
                                             $(,•$rest:tt)*             MacroGroup
                                               ,                        PunctuationToken
                                                 $rest:tt               MacroParameterDeclaration
                                                 $rest                  McIdentifier                                                      */
    (impl $fblock:tt [$($blocks:tt,)*] []) => {};                                                                                         /*
    (impl•$fblock:tt•[$($blocks:tt,)*]•[])•=>•{}     MacroRuleDeclaration
          $fblock:tt                                 MacroParameterDeclaration
          $fblock                                    McIdentifier
                     [$($blocks:tt,)*]               DelimGroup
                      $($blocks:tt,)*                MacroGroup
                        $blocks:tt                   MacroParameterDeclaration
                        $blocks                      McIdentifier
                                  ,                  PunctuationToken
                                       []            DelimGroup                                                                           */
    ($($ecode:ident: $message:expr,)* ; $($code:ident,)*) => {};                                                                          /*
    ($($ecode:ident:•$message:expr,)*•;•$($code:ident,)*)•=>•{}     MacroRuleDeclaration
     $($ecode:ident:•$message:expr,)*                               MacroGroup
       $ecode:ident                                                 MacroParameterDeclaration
       $ecode                                                       McIdentifier
                   :                                                PunctuationToken
                     $message:expr                                  MacroParameterDeclaration
                     $message                                       McIdentifier
                                  ,                                 PunctuationToken
                                      ;                             PunctuationToken
                                        $($code:ident,)*            MacroGroup
                                          $code:ident               MacroParameterDeclaration
                                          $code                     McIdentifier
                                                     ,              PunctuationToken                                                      */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $(#[$attrs:meta])*                                                                                                                /*
        $(#[$attrs:meta])*    MacroGroup
          #                   PunctuationToken
           [$attrs:meta]      DelimGroup
            $attrs:meta       MacroParameterDeclaration
            $attrs            McIdentifier                                                                                                */
        pub fn $n:ident(&self, $($name:ident: $ty:ty),* $(,)?) -> &Self                                                                   /*
               $n:ident                                                    MacroParameterDeclaration
               $n                                                          McIdentifier
                       (&self,•$($name:ident:•$ty:ty),*•$(,)?)             DelimGroup
                        &                                                  PunctuationToken
                             ,                                             PunctuationToken
                               $($name:ident:•$ty:ty),*                    MacroGroup
                                 $name:ident                               MacroParameterDeclaration
                                 $name                                     McIdentifier
                                            :                              PunctuationToken
                                              $ty:ty                       MacroParameterDeclaration
                                              $ty                          McIdentifier
                                                     ,                     PunctuationToken
                                                        $(,)?              MacroGroup
                                                          ,                PunctuationToken
                                                               ->          PunctuationToken
                                                                  &        PunctuationToken                                               */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $enc:expr,                  // _______
                                                                                                                                          /*
        $enc:expr                                 MacroParameterDeclaration
        $enc                                      McIdentifier
                 ,                                PunctuationToken
                                    //•_______    Comment                                                                                 */
        $idx:expr,                  // _______
                                                                                                                                          /*
        $idx:expr                                 MacroParameterDeclaration
        $idx                                      McIdentifier
                 ,                                PunctuationToken
                                    //•_______    Comment                                                                                 */
        $struct:expr,               // _______
                                                                                                                                          /*
        $struct:expr                              MacroParameterDeclaration
        $struct                                   McIdentifier
                    ,                             PunctuationToken
                                    //•_______    Comment                                                                                 */
        $struct_name:ident,         // _______
                                                                                                                                          /*
        $struct_name:ident                        MacroParameterDeclaration
        $struct_name                              McIdentifier
                          ,                       PunctuationToken
                                    //•_______    Comment                                                                                 */
        [ $($name:ident),+$(,)? ],  // _______
                                                                                                                                          /*
        [•$($name:ident),+$(,)?•]                 DelimGroup
          $($name:ident),+                        MacroGroup
            $name:ident                           MacroParameterDeclaration
            $name                                 McIdentifier
                        ,                         PunctuationToken
                          $(,)?                   MacroGroup
                            ,                     PunctuationToken
                                 ,                PunctuationToken
                                    //•_______    Comment                                                                                 */
        [ $($ignore:ident),+$(,)? ] // _______
                                                                                                                                          /*
        [•$($ignore:ident),+$(,)?•]               DelimGroup
          $($ignore:ident),+                      MacroGroup
            $ignore:ident                         MacroParameterDeclaration
            $ignore                               McIdentifier
                          ,                       PunctuationToken
                            $(,)?                 MacroGroup
                              ,                   PunctuationToken
                                    //•_______    Comment                                                                                 */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $($Kind:ident($AstTy:ty) {                                                                                                        /*
        $($Kind:ident($AstTy:ty)•{↲    <MacroGroup>
          $Kind:ident                  MacroParameterDeclaration
          $Kind                        McIdentifier
                     ($AstTy:ty)       DelimGroup
                      $AstTy:ty        MacroParameterDeclaration
                      $AstTy           McIdentifier
                                 {↲    <DelimGroup>                                                                                       */
            $kind_name:expr;                                                                                                              /*
            $kind_name:expr     MacroParameterDeclaration
            $kind_name          McIdentifier
                           ;    PunctuationToken                                                                                          */
            $(one fn $mut_visit_ast:ident; fn $visit_ast:ident;)?                                                                         /*
            $(one•fn•$mut_visit_ast:ident;•fn•$visit_ast:ident;)?    MacroGroup
                     $mut_visit_ast:ident                            MacroParameterDeclaration
                     $mut_visit_ast                                  McIdentifier
                                         ;                           PunctuationToken
                                              $visit_ast:ident       MacroParameterDeclaration
                                              $visit_ast             McIdentifier
                                                              ;      PunctuationToken                                                     */
            $(many fn $flat_map_ast_elt:ident; fn $visit_ast_elt:ident($($args:tt)*);)?                                                   /*
            $(many•fn•$flat_map_ast_elt:ident;•fn•$visit_ast_elt:ident($($args:tt)*);)?    MacroGroup
                      $flat_map_ast_elt:ident                                              MacroParameterDeclaration
                      $flat_map_ast_elt                                                    McIdentifier
                                             ;                                             PunctuationToken
                                                  $visit_ast_elt:ident                     MacroParameterDeclaration
                                                  $visit_ast_elt                           McIdentifier
                                                                      ($($args:tt)*)       DelimGroup
                                                                       $($args:tt)*        MacroGroup
                                                                         $args:tt          MacroParameterDeclaration
                                                                         $args             McIdentifier
                                                                                    ;      PunctuationToken                               */
            fn $make_ast:ident;                                                                                                           /*
               $make_ast:ident     MacroParameterDeclaration
               $make_ast           McIdentifier
                              ;    PunctuationToken                                                                                       */
        })*                                                                                                                               /*
••••••••})*    </MacroGroup>
••••••••}      </DelimGroup>                                                                                                              */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    ($ty:ident { $($field:ident $(: $value:expr)*),+ $(,)? }) => {};                                                                      /*
    ($ty:ident•{•$($field:ident•$(:•$value:expr)*),+•$(,)?•})•=>•{}     MacroRuleDeclaration
     $ty:ident                                                          MacroParameterDeclaration
     $ty                                                                McIdentifier
               {•$($field:ident•$(:•$value:expr)*),+•$(,)?•}            DelimGroup
                 $($field:ident•$(:•$value:expr)*),+                    MacroGroup
                   $field:ident                                         MacroParameterDeclaration
                   $field                                               McIdentifier
                                $(:•$value:expr)*                       MacroGroup
                                  :                                     PunctuationToken
                                    $value:expr                         MacroParameterDeclaration
                                    $value                              McIdentifier
                                                  ,                     PunctuationToken
                                                     $(,)?              MacroGroup
                                                       ,                PunctuationToken                                                  */
    ($ty:ident::$method:ident($($value:expr),*)) => {};                                                                                   /*
    ($ty:ident::$method:ident($($value:expr),*))•=>•{}     MacroRuleDeclaration
     $ty:ident                                             MacroParameterDeclaration
     $ty                                                   McIdentifier
              ::                                           PunctuationToken
                $method:ident                              MacroParameterDeclaration
                $method                                    McIdentifier
                             ($($value:expr),*)            DelimGroup
                              $($value:expr),*             MacroGroup
                                $value:expr                MacroParameterDeclaration
                                $value                     McIdentifier
                                            ,              PunctuationToken                                                               */
    ($(                                                                                                                                   /*
    ($(↲    <MacroRuleDeclaration>
     $(↲    <MacroGroup>                                                                                                                  */
        $(#[doc = $doc:tt])* (accepted, $feature:ident, $ver:expr, $issue:expr, None),                                                    /*
        $(#[doc•=•$doc:tt])*                                                              MacroGroup
          #                                                                               PunctuationToken
           [doc•=•$doc:tt]                                                                DelimGroup
                =                                                                         PunctuationToken
                  $doc:tt                                                                 MacroParameterDeclaration
                  $doc                                                                    McIdentifier
                             (accepted,•$feature:ident,•$ver:expr,•$issue:expr,•None)     DelimGroup
                                      ,                                                   PunctuationToken
                                        $feature:ident                                    MacroParameterDeclaration
                                        $feature                                          McIdentifier
                                                      ,                                   PunctuationToken
                                                        $ver:expr                         MacroParameterDeclaration
                                                        $ver                              McIdentifier
                                                                 ,                        PunctuationToken
                                                                   $issue:expr            MacroParameterDeclaration
                                                                   $issue                 McIdentifier
                                                                              ,           PunctuationToken
                                                                                     ,    PunctuationToken                                */
    )+) => {};                                                                                                                            /*
••••)+)•=>•{}     </MacroRuleDeclaration>
••••)+            </MacroGroup>                                                                                                           */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $( $(#[$attr:meta])* $variant:ident $($group:expr)?, $module:ident :: $name:ident, $method:ident, $target:expr, $generics:expr; )*/*
        $(•$(#[$attr:meta])*•$variant:ident•$($group:expr)?,•$module:ident•::•$name:ident,•$method:ident,•$target:expr,•$generics:expr;•)*    MacroGroup
           $(#[$attr:meta])*                                                                                                                  MacroGroup
             #                                                                                                                                PunctuationToken
              [$attr:meta]                                                                                                                    DelimGroup
               $attr:meta                                                                                                                     MacroParameterDeclaration
               $attr                                                                                                                          McIdentifier
                             $variant:ident                                                                                                   MacroParameterDeclaration
                             $variant                                                                                                         McIdentifier
                                            $($group:expr)?                                                                                   MacroGroup
                                              $group:expr                                                                                     MacroParameterDeclaration
                                              $group                                                                                          McIdentifier
                                                           ,                                                                                  PunctuationToken
                                                             $module:ident                                                                    MacroParameterDeclaration
                                                             $module                                                                          McIdentifier
                                                                           ::                                                                 PunctuationToken
                                                                              $name:ident                                                     MacroParameterDeclaration
                                                                              $name                                                           McIdentifier
                                                                                         ,                                                    PunctuationToken
                                                                                           $method:ident                                      MacroParameterDeclaration
                                                                                           $method                                            McIdentifier
                                                                                                        ,                                     PunctuationToken
                                                                                                          $target:expr                        MacroParameterDeclaration
                                                                                                          $target                             McIdentifier
                                                                                                                      ,                       PunctuationToken
                                                                                                                        $generics:expr        MacroParameterDeclaration
                                                                                                                        $generics             McIdentifier
                                                                                                                                      ;       PunctuationToken*/
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    ([$($(#[$attr:meta])* fn $name:ident($($param:ident: $arg:ty),*);)*]) => {};                                                          /*
    ([$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])•=>•{}     MacroRuleDeclaration
     [$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*]            DelimGroup
      $($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*             MacroGroup
        $(#[$attr:meta])*                                                           MacroGroup
          #                                                                         PunctuationToken
           [$attr:meta]                                                             DelimGroup
            $attr:meta                                                              MacroParameterDeclaration
            $attr                                                                   McIdentifier
                             $name:ident                                            MacroParameterDeclaration
                             $name                                                  McIdentifier
                                        ($($param:ident:•$arg:ty),*)                DelimGroup
                                         $($param:ident:•$arg:ty),*                 MacroGroup
                                           $param:ident                             MacroParameterDeclaration
                                           $param                                   McIdentifier
                                                       :                            PunctuationToken
                                                         $arg:ty                    MacroParameterDeclaration
                                                         $arg                       McIdentifier
                                                                 ,                  PunctuationToken
                                                                    ;               PunctuationToken                                      */
    ([$hir:tt], [$($(#[$attr:meta])* fn $name:ident($($param:ident: $arg:ty),*);)*]) => {};                                               /*
    ([$hir:tt],•[$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*])•=>•{}     MacroRuleDeclaration
     [$hir:tt]                                                                                 DelimGroup
      $hir:tt                                                                                  MacroParameterDeclaration
      $hir                                                                                     McIdentifier
              ,                                                                                PunctuationToken
                [$($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*]            DelimGroup
                 $($(#[$attr:meta])*•fn•$name:ident($($param:ident:•$arg:ty),*);)*             MacroGroup
                   $(#[$attr:meta])*                                                           MacroGroup
                     #                                                                         PunctuationToken
                      [$attr:meta]                                                             DelimGroup
                       $attr:meta                                                              MacroParameterDeclaration
                       $attr                                                                   McIdentifier
                                        $name:ident                                            MacroParameterDeclaration
                                        $name                                                  McIdentifier
                                                   ($($param:ident:•$arg:ty),*)                DelimGroup
                                                    $($param:ident:•$arg:ty),*                 MacroGroup
                                                      $param:ident                             MacroParameterDeclaration
                                                      $param                                   McIdentifier
                                                                  :                            PunctuationToken
                                                                    $arg:ty                    MacroParameterDeclaration
                                                                    $arg                       McIdentifier
                                                                            ,                  PunctuationToken
                                                                               ;               PunctuationToken                           */
    ([$span:expr, $($fmt:tt)*] $arg:expr, $($rest:tt)*) => {};                                                                            /*
    ([$span:expr,•$($fmt:tt)*]•$arg:expr,•$($rest:tt)*)•=>•{}     MacroRuleDeclaration
     [$span:expr,•$($fmt:tt)*]                                    DelimGroup
      $span:expr                                                  MacroParameterDeclaration
      $span                                                       McIdentifier
                ,                                                 PunctuationToken
                  $($fmt:tt)*                                     MacroGroup
                    $fmt:tt                                       MacroParameterDeclaration
                    $fmt                                          McIdentifier
                               $arg:expr                          MacroParameterDeclaration
                               $arg                               McIdentifier
                                        ,                         PunctuationToken
                                          $($rest:tt)*            MacroGroup
                                            $rest:tt              MacroParameterDeclaration
                                            $rest                 McIdentifier                                                            */
    ($($T:ty),*) => {};                                                                                                                   /*
    ($($T:ty),*)•=>•{}     MacroRuleDeclaration
     $($T:ty),*            MacroGroup
       $T:ty               MacroParameterDeclaration
       $T                  McIdentifier
             ,             PunctuationToken                                                                                               */
    (# $var:ident) => {};                                                                                                                 /*
    (#•$var:ident)•=>•{}     MacroRuleDeclaration
     #                       PunctuationToken
       $var:ident            MacroParameterDeclaration
       $var                  McIdentifier                                                                                                 */
    ($call:ident! $extra:tt ($($b1:tt)*) ($($curr:tt)*)) => {};                                                                           /*
    ($call:ident!•$extra:tt•($($b1:tt)*)•($($curr:tt)*))•=>•{}     MacroRuleDeclaration
     $call:ident                                                   MacroParameterDeclaration
     $call                                                         McIdentifier
                !                                                  PunctuationToken
                  $extra:tt                                        MacroParameterDeclaration
                  $extra                                           McIdentifier
                            ($($b1:tt)*)                           DelimGroup
                             $($b1:tt)*                            MacroGroup
                               $b1:tt                              MacroParameterDeclaration
                               $b1                                 McIdentifier
                                         ($($curr:tt)*)            DelimGroup
                                          $($curr:tt)*             MacroGroup
                                            $curr:tt               MacroParameterDeclaration
                                            $curr                  McIdentifier                                                           */
    ($call:ident!($($extra:tt)*) # $var:ident) => {};                                                                                     /*
    ($call:ident!($($extra:tt)*)•#•$var:ident)•=>•{}     MacroRuleDeclaration
     $call:ident                                         MacroParameterDeclaration
     $call                                               McIdentifier
                !                                        PunctuationToken
                 ($($extra:tt)*)                         DelimGroup
                  $($extra:tt)*                          MacroGroup
                    $extra:tt                            MacroParameterDeclaration
                    $extra                               McIdentifier
                                 #                       PunctuationToken
                                   $var:ident            MacroParameterDeclaration
                                   $var                  McIdentifier                                                                     */
    ($tokens:ident                                                                                                                        /*
    ($tokens:ident↲    <MacroRuleDeclaration>
     $tokens:ident     MacroParameterDeclaration
     $tokens           McIdentifier                                                                                                       */
        ($($b3:tt)*) ($($b2:tt)*) ($($b1:tt)*)                                                                                            /*
        ($($b3:tt)*)                              DelimGroup
         $($b3:tt)*                               MacroGroup
           $b3:tt                                 MacroParameterDeclaration
           $b3                                    McIdentifier
                     ($($b2:tt)*)                 DelimGroup
                      $($b2:tt)*                  MacroGroup
                        $b2:tt                    MacroParameterDeclaration
                        $b2                       McIdentifier
                                  ($($b1:tt)*)    DelimGroup
                                   $($b1:tt)*     MacroGroup
                                     $b1:tt       MacroParameterDeclaration
                                     $b1          McIdentifier                                                                            */
        ($($curr:tt)*)                                                                                                                    /*
        ($($curr:tt)*)    DelimGroup
         $($curr:tt)*     MacroGroup
           $curr:tt       MacroParameterDeclaration
           $curr          McIdentifier                                                                                                    */
        ($($a1:tt)*) ($($a2:tt)*) ($($a3:tt)*)                                                                                            /*
        ($($a1:tt)*)                              DelimGroup
         $($a1:tt)*                               MacroGroup
           $a1:tt                                 MacroParameterDeclaration
           $a1                                    McIdentifier
                     ($($a2:tt)*)                 DelimGroup
                      $($a2:tt)*                  MacroGroup
                        $a2:tt                    MacroParameterDeclaration
                        $a2                       McIdentifier
                                  ($($a3:tt)*)    DelimGroup
                                   $($a3:tt)*     MacroGroup
                                     $a3:tt       MacroParameterDeclaration
                                     $a3          McIdentifier                                                                            */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    ($tokens:ident $b3:tt $b2:tt $b1:tt @ $a1:tt $a2:tt $a3:tt) => {};                                                                    /*
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•@•$a1:tt•$a2:tt•$a3:tt)•=>•{}     MacroRuleDeclaration
     $tokens:ident                                                        MacroParameterDeclaration
     $tokens                                                              McIdentifier
                   $b3:tt                                                 MacroParameterDeclaration
                   $b3                                                    McIdentifier
                          $b2:tt                                          MacroParameterDeclaration
                          $b2                                             McIdentifier
                                 $b1:tt                                   MacroParameterDeclaration
                                 $b1                                      McIdentifier
                                        @                                 PunctuationToken
                                          $a1:tt                          MacroParameterDeclaration
                                          $a1                             McIdentifier
                                                 $a2:tt                   MacroParameterDeclaration
                                                 $a2                      McIdentifier
                                                        $a3:tt            MacroParameterDeclaration
                                                        $a3               McIdentifier                                                    */
    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) * $a3:tt) => {};                                                            /*
    ($tokens:ident•$b3:tt•$b2:tt•$b1:tt•(#)•(•$($inner:tt)*•)•*•$a3:tt)•=>•{}     MacroRuleDeclaration
     $tokens:ident                                                                MacroParameterDeclaration
     $tokens                                                                      McIdentifier
                   $b3:tt                                                         MacroParameterDeclaration
                   $b3                                                            McIdentifier
                          $b2:tt                                                  MacroParameterDeclaration
                          $b2                                                     McIdentifier
                                 $b1:tt                                           MacroParameterDeclaration
                                 $b1                                              McIdentifier
                                        (#)                                       DelimGroup
                                         #                                        PunctuationToken
                                            (•$($inner:tt)*•)                     DelimGroup
                                              $($inner:tt)*                       MacroGroup
                                                $inner:tt                         MacroParameterDeclaration
                                                $inner                            McIdentifier
                                                              *                   PunctuationToken
                                                                $a3:tt            MacroParameterDeclaration
                                                                $a3               McIdentifier                                            */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        [$($attrs_pub:tt)*]                                                                                                               /*
        [$($attrs_pub:tt)*]    DelimGroup
         $($attrs_pub:tt)*     MacroGroup
           $attrs_pub:tt       MacroParameterDeclaration
           $attrs_pub          McIdentifier                                                                                               */
        enum $name:ident #no_visit $($rest:tt)*                                                                                           /*
             $name:ident                           MacroParameterDeclaration
             $name                                 McIdentifier
                         #                         PunctuationToken
                                   $($rest:tt)*    MacroGroup
                                     $rest:tt      MacroParameterDeclaration
                                     $rest         McIdentifier                                                                           */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $(#[$enum_attr:meta])*                                                                                                            /*
        $(#[$enum_attr:meta])*    MacroGroup
          #                       PunctuationToken
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
                                           #                       PunctuationToken
                                            $tag:ident             MacroParameterDeclaration
                                            $tag                   McIdentifier
                                                       $body:tt    MacroParameterDeclaration
                                                       $body       McIdentifier                                                           */
        $($remaining:tt)*                                                                                                                 /*
        $($remaining:tt)*    MacroGroup
          $remaining:tt      MacroParameterDeclaration
          $remaining         McIdentifier                                                                                                 */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (                                                                                                                                     /*
    (↲    <MacroRuleDeclaration>                                                                                                          */
        $pub:ident $enum:ident $name:ident {                                                                                              /*
        $pub:ident                               MacroParameterDeclaration
        $pub                                     McIdentifier
                   $enum:ident                   MacroParameterDeclaration
                   $enum                         McIdentifier
                               $name:ident       MacroParameterDeclaration
                               $name             McIdentifier
                                           {↲    <DelimGroup>                                                                             */
            $(                                                                                                                            /*
            $(↲    <MacroGroup>                                                                                                           */
                $(#[$variant_attr:meta])*                                                                                                 /*
                $(#[$variant_attr:meta])*    MacroGroup
                  #                          PunctuationToken
                   [$variant_attr:meta]      DelimGroup
                    $variant_attr:meta       MacroParameterDeclaration
                    $variant_attr            McIdentifier                                                                                 */
                $variant:ident $( ($($member:ident)::+) )*,                                                                               /*
                $variant:ident                                 MacroParameterDeclaration
                $variant                                       McIdentifier
                               $(•($($member:ident)::+)•)*     MacroGroup
                                  ($($member:ident)::+)        DelimGroup
                                   $($member:ident)::+         MacroGroup
                                     $member:ident             MacroParameterDeclaration
                                     $member                   McIdentifier
                                                   ::          PunctuationToken
                                                          ,    PunctuationToken                                                           */
            )*                                                                                                                            /*
••••••••••••)*    </MacroGroup>                                                                                                           */
        }                                                                                                                                 /*
••••••••}    </DelimGroup>                                                                                                                */

        $($remaining:tt)*                                                                                                                 /*
        $($remaining:tt)*    MacroGroup
          $remaining:tt      MacroParameterDeclaration
          $remaining         McIdentifier                                                                                                 */
    ) => {};                                                                                                                              /*
••••)•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (($($arms:tt)*) $tokens:ident $name:ident { $variant:ident $member:ident, $($next:tt)*}) => {};                                       /*
    (($($arms:tt)*)•$tokens:ident•$name:ident•{•$variant:ident•$member:ident,•$($next:tt)*})•=>•{}     MacroRuleDeclaration
     ($($arms:tt)*)                                                                                    DelimGroup
      $($arms:tt)*                                                                                     MacroGroup
        $arms:tt                                                                                       MacroParameterDeclaration
        $arms                                                                                          McIdentifier
                    $tokens:ident                                                                      MacroParameterDeclaration
                    $tokens                                                                            McIdentifier
                                  $name:ident                                                          MacroParameterDeclaration
                                  $name                                                                McIdentifier
                                              {•$variant:ident•$member:ident,•$($next:tt)*}            DelimGroup
                                                $variant:ident                                         MacroParameterDeclaration
                                                $variant                                               McIdentifier
                                                               $member:ident                           MacroParameterDeclaration
                                                               $member                                 McIdentifier
                                                                            ,                          PunctuationToken
                                                                              $($next:tt)*             MacroGroup
                                                                                $next:tt               MacroParameterDeclaration
                                                                                $next                  McIdentifier                       */
    ($mac:ident!($(#[$m:meta])* $pub:ident $($t:tt)*)) => {};                                                                             /*
    ($mac:ident!($(#[$m:meta])*•$pub:ident•$($t:tt)*))•=>•{}     MacroRuleDeclaration
     $mac:ident                                                  MacroParameterDeclaration
     $mac                                                        McIdentifier
               !                                                 PunctuationToken
                ($(#[$m:meta])*•$pub:ident•$($t:tt)*)            DelimGroup
                 $(#[$m:meta])*                                  MacroGroup
                   #                                             PunctuationToken
                    [$m:meta]                                    DelimGroup
                     $m:meta                                     MacroParameterDeclaration
                     $m                                          McIdentifier
                                $pub:ident                       MacroParameterDeclaration
                                $pub                             McIdentifier
                                           $($t:tt)*             MacroGroup
                                             $t:tt               MacroParameterDeclaration
                                             $t                  McIdentifier                                                             */
    ($($token:tt pub struct $name:ident #[$doc:meta])*) => {};                                                                            /*
    ($($token:tt•pub•struct•$name:ident•#[$doc:meta])*)•=>•{}     MacroRuleDeclaration
     $($token:tt•pub•struct•$name:ident•#[$doc:meta])*            MacroGroup
       $token:tt                                                  MacroParameterDeclaration
       $token                                                     McIdentifier
                            $name:ident                           MacroParameterDeclaration
                            $name                                 McIdentifier
                                        #                         PunctuationToken
                                         [$doc:meta]              DelimGroup
                                          $doc:meta               MacroParameterDeclaration
                                          $doc                    McIdentifier                                                            */
    ($($token:tt pub struct $name:ident/$len:tt #[$doc:meta])*) => {};                                                                    /*
    ($($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])*)•=>•{}     MacroRuleDeclaration
     $($token:tt•pub•struct•$name:ident/$len:tt•#[$doc:meta])*            MacroGroup
       $token:tt                                                          MacroParameterDeclaration
       $token                                                             McIdentifier
                            $name:ident                                   MacroParameterDeclaration
                            $name                                         McIdentifier
                                       /                                  PunctuationToken
                                        $len:tt                           MacroParameterDeclaration
                                        $len                              McIdentifier
                                                #                         PunctuationToken
                                                 [$doc:meta]              DelimGroup
                                                  $doc:meta               MacroParameterDeclaration
                                                  $doc                    McIdentifier                                                    */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>                                                                                                          */
        $($name:ident)::+ $(<$param:ident>)?                                                                                              /*
        $($name:ident)::+                       MacroGroup
          $name:ident                           MacroParameterDeclaration
          $name                                 McIdentifier
                      ::                        PunctuationToken
                          $(<$param:ident>)?    MacroGroup
                            <                   PunctuationToken
                             $param:ident       MacroParameterDeclaration
                             $param             McIdentifier
                                         >      PunctuationToken                                                                          */
        $([$field:tt $this:ident $other:ident])*                                                                                          /*
        $([$field:tt•$this:ident•$other:ident])*    MacroGroup
          [$field:tt•$this:ident•$other:ident]      DelimGroup
           $field:tt                                MacroParameterDeclaration
           $field                                   McIdentifier
                     $this:ident                    MacroParameterDeclaration
                     $this                          McIdentifier
                                 $other:ident       MacroParameterDeclaration
                                 $other             McIdentifier                                                                          */
        $(![$ignore:tt])*;                                                                                                                /*
        $(![$ignore:tt])*     MacroGroup
          !                   PunctuationToken
           [$ignore:tt]       DelimGroup
            $ignore:tt        MacroParameterDeclaration
            $ignore           McIdentifier
                         ;    PunctuationToken                                                                                            */
        !$next:tt                                                                                                                         /*
        !            PunctuationToken
         $next:tt    MacroParameterDeclaration
         $next       McIdentifier                                                                                                         */
        $($rest:tt)*                                                                                                                      /*
        $($rest:tt)*    MacroGroup
          $rest:tt      MacroParameterDeclaration
          $rest         McIdentifier                                                                                                      */
    } => {};                                                                                                                              /*
••••}•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>                                                                                                          */
        $($name:ident)::+;                                                                                                                /*
        $($name:ident)::+     MacroGroup
          $name:ident         MacroParameterDeclaration
          $name               McIdentifier
                      ::      PunctuationToken
                         ;    PunctuationToken                                                                                            */
        $([$($variant:ident)::+; $([$field:tt $this:ident $other:ident])* $(![$ignore:tt])*])*                                            /*
        $([$($variant:ident)::+;•$([$field:tt•$this:ident•$other:ident])*•$(![$ignore:tt])*])*    MacroGroup
          [$($variant:ident)::+;•$([$field:tt•$this:ident•$other:ident])*•$(![$ignore:tt])*]      DelimGroup
           $($variant:ident)::+                                                                   MacroGroup
             $variant:ident                                                                       MacroParameterDeclaration
             $variant                                                                             McIdentifier
                            ::                                                                    PunctuationToken
                               ;                                                                  PunctuationToken
                                 $([$field:tt•$this:ident•$other:ident])*                         MacroGroup
                                   [$field:tt•$this:ident•$other:ident]                           DelimGroup
                                    $field:tt                                                     MacroParameterDeclaration
                                    $field                                                        McIdentifier
                                              $this:ident                                         MacroParameterDeclaration
                                              $this                                               McIdentifier
                                                          $other:ident                            MacroParameterDeclaration
                                                          $other                                  McIdentifier
                                                                          $(![$ignore:tt])*       MacroGroup
                                                                            !                     PunctuationToken
                                                                             [$ignore:tt]         DelimGroup
                                                                              $ignore:tt          MacroParameterDeclaration
                                                                              $ignore             McIdentifier                            */
    } => {};                                                                                                                              /*
••••}•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    {                                                                                                                                     /*
    {↲    <MacroRuleDeclaration>                                                                                                          */
        $($name:ident)::+;                                                                                                                /*
        $($name:ident)::+     MacroGroup
          $name:ident         MacroParameterDeclaration
          $name               McIdentifier
                      ::      PunctuationToken
                         ;    PunctuationToken                                                                                            */
        $([$($variant:ident)::+; $($fields:tt)*])*                                                                                        /*
        $([$($variant:ident)::+;•$($fields:tt)*])*    MacroGroup
          [$($variant:ident)::+;•$($fields:tt)*]      DelimGroup
           $($variant:ident)::+                       MacroGroup
             $variant:ident                           MacroParameterDeclaration
             $variant                                 McIdentifier
                            ::                        PunctuationToken
                               ;                      PunctuationToken
                                 $($fields:tt)*       MacroGroup
                                   $fields:tt         MacroParameterDeclaration
                                   $fields            McIdentifier                                                                        */
        $next:ident                                                                                                                       /*
        $next:ident    MacroParameterDeclaration
        $next          McIdentifier                                                                                                       */
        $($rest:tt)*                                                                                                                      /*
        $($rest:tt)*    MacroGroup
          $rest:tt      MacroParameterDeclaration
          $rest         McIdentifier                                                                                                      */
    } => {};                                                                                                                              /*
••••}•=>•{}     </MacroRuleDeclaration>                                                                                                   */
    (($expr:ident) as $t:ty, @$snapshot:literal) => {};                                                                                   /*
    (($expr:ident)•as•$t:ty,•@$snapshot:literal)•=>•{}     MacroRuleDeclaration
     ($expr:ident)                                         DelimGroup
      $expr:ident                                          MacroParameterDeclaration
      $expr                                                McIdentifier
                      $t:ty                                MacroParameterDeclaration
                      $t                                   McIdentifier
                           ,                               PunctuationToken
                             @                             PunctuationToken
                              $snapshot:literal            MacroParameterDeclaration
                              $snapshot                    McIdentifier                                                                   */
    ($b : block, $e : expr, $i : ident, $it : item, $l : lifetime, $lit :                                                                 /*
    ($b•:•block,•$e•:•expr,•$i•:•ident,•$it•:•item,•$l•:•lifetime,•$lit•:↲    <MacroRuleDeclaration>
     $b•:•block                                                               MacroParameterDeclaration
     $b                                                                       McIdentifier
               ,                                                              PunctuationToken
                 $e•:•expr                                                    MacroParameterDeclaration
                 $e                                                           McIdentifier
                          ,                                                   PunctuationToken
                            $i•:•ident                                        MacroParameterDeclaration
                            $i                                                McIdentifier
                                      ,                                       PunctuationToken
                                        $it•:•item                            MacroParameterDeclaration
                                        $it                                   McIdentifier
                                                  ,                           PunctuationToken
                                                    $l•:•lifetime             MacroParameterDeclaration
                                                    $l                        McIdentifier
                                                                 ,            PunctuationToken
                                                                   $lit•:↲    <MacroParameterDeclaration>
                                                                   $lit       McIdentifier                                                */
     literal, $m : meta, $p : pat, $pth : path, $s : stmt, $tt : tt, $ty : ty,                                                            /*
•••••literal                                                                      </MacroParameterDeclaration>
            ,                                                                     PunctuationToken
              $m•:•meta                                                           MacroParameterDeclaration
              $m                                                                  McIdentifier
                       ,                                                          PunctuationToken
                         $p•:•pat                                                 MacroParameterDeclaration
                         $p                                                       McIdentifier
                                 ,                                                PunctuationToken
                                   $pth•:•path                                    MacroParameterDeclaration
                                   $pth                                           McIdentifier
                                              ,                                   PunctuationToken
                                                $s•:•stmt                         MacroParameterDeclaration
                                                $s                                McIdentifier
                                                         ,                        PunctuationToken
                                                           $tt•:•tt               MacroParameterDeclaration
                                                           $tt                    McIdentifier
                                                                   ,              PunctuationToken
                                                                     $ty•:•ty     MacroParameterDeclaration
                                                                     $ty          McIdentifier
                                                                             ,    PunctuationToken                                        */
     $vis : vis) => { } ;                                                                                                                 /*
•••••$vis•:•vis)•=>•{•}      </MacroRuleDeclaration>
     $vis•:•vis              MacroParameterDeclaration
     $vis                    McIdentifier                                                                                                 */
	($(#[$smeta:meta])* pub struct $stratname:ident [$($sgen:tt)*][$($swhere:tt)*]                                                        /*
    ($(#[$smeta:meta])*•pub•struct•$stratname:ident•[$($sgen:tt)*][$($swhere:tt)*]•↲    <MacroRuleDeclaration>
     $(#[$smeta:meta])*                                                                 MacroGroup
       #                                                                                PunctuationToken
        [$smeta:meta]                                                                   DelimGroup
         $smeta:meta                                                                    MacroParameterDeclaration
         $smeta                                                                         McIdentifier
                                   $stratname:ident                                     MacroParameterDeclaration
                                   $stratname                                           McIdentifier
                                                    [$($sgen:tt)*]                      DelimGroup
                                                     $($sgen:tt)*                       MacroGroup
                                                       $sgen:tt                         MacroParameterDeclaration
                                                       $sgen                            McIdentifier
                                                                  [$($swhere:tt)*]      DelimGroup
                                                                   $($swhere:tt)*       MacroGroup
                                                                     $swhere:tt         MacroParameterDeclaration
                                                                     $swhere            McIdentifier                                      */
	($innerstrat:ty) -> $stratvtty:ty; $(#[$vmeta:meta])* pub struct $vtname:ident                                                        /*
    ($innerstrat:ty)                                                                  DelimGroup
     $innerstrat:ty                                                                   MacroParameterDeclaration
     $innerstrat                                                                      McIdentifier
                     ->                                                               PunctuationToken
                        $stratvtty:ty                                                 MacroParameterDeclaration
                        $stratvtty                                                    McIdentifier
                                     ;                                                PunctuationToken
                                       $(#[$vmeta:meta])*                             MacroGroup
                                         #                                            PunctuationToken
                                          [$vmeta:meta]                               DelimGroup
                                           $vmeta:meta                                MacroParameterDeclaration
                                           $vmeta                                     McIdentifier
                                                                     $vtname:ident    MacroParameterDeclaration
                                                                     $vtname          McIdentifier                                        */
	[$($vgen:tt)*][$($vwhere:tt)*] ($innervt:ty) -> $actualty:ty; ) => {}                                                                 /*
   ╚[$($vgen:tt)*][$($vwhere:tt)*]•($innervt:ty)•->•$actualty:ty;•)•=>•{}    </MacroRuleDeclaration>
    [$($vgen:tt)*]                                                           DelimGroup
     $($vgen:tt)*                                                            MacroGroup
       $vgen:tt                                                              MacroParameterDeclaration
       $vgen                                                                 McIdentifier
                  [$($vwhere:tt)*]                                           DelimGroup
                   $($vwhere:tt)*                                            MacroGroup
                     $vwhere:tt                                              MacroParameterDeclaration
                     $vwhere                                                 McIdentifier
                                   ($innervt:ty)                             DelimGroup
                                    $innervt:ty                              MacroParameterDeclaration
                                    $innervt                                 McIdentifier
                                                 ->                          PunctuationToken
                                                    $actualty:ty             MacroParameterDeclaration
                                                    $actualty                McIdentifier
                                                                ;            PunctuationToken                                             */

    {  } => { Nil } ;                                                                                                                     /*
    {••}•=>•{•Nil•}      MacroRuleDeclaration                                                                                             */
    { $ head : expr } => { Cons ( $ head , Nil ) } ;                                                                                      /*
    {•$•head•:•expr•}•=>•{•Cons•(•$•head•,•Nil•)•}      MacroRuleDeclaration
      $                                                 PunctuationToken
             :                                          PunctuationToken
                                (•$•head•,•Nil•)        DelimGroup
                                  $                     PunctuationToken
                                         ,              PunctuationToken                                                                  */
    { $ head : expr , $ ( $ tail : expr ) , * } => { Cons ( $ head , hlist ! ( $ ( $ tail ) , * ) ) } ;                                   /*
    {•$•head•:•expr•,•$•(•$•tail•:•expr•)•,•*•}•=>•{•Cons•(•$•head•,•hlist•!•(•$•(•$•tail•)•,•*•)•)•}      MacroRuleDeclaration
      $                                                                                                    PunctuationToken
             :                                                                                             PunctuationToken
                    ,                                                                                      PunctuationToken
                      $                                                                                    PunctuationToken
                        (•$•tail•:•expr•)                                                                  DelimGroup
                          $                                                                                PunctuationToken
                                 :                                                                         PunctuationToken
                                          ,                                                                PunctuationToken
                                            *                                                              PunctuationToken
                                                          (•$•head•,•hlist•!•(•$•(•$•tail•)•,•*•)•)        DelimGroup
                                                            $                                              PunctuationToken
                                                                   ,                                       PunctuationToken
                                                                           !                               PunctuationToken
                                                                             (•$•(•$•tail•)•,•*•)          DelimGroup
                                                                               $                           PunctuationToken
                                                                                 (•$•tail•)                DelimGroup
                                                                                   $                       PunctuationToken
                                                                                            ,              PunctuationToken
                                                                                              *            PunctuationToken               */
    { $ head : ty } => { Cons < $ head , Nil > } ;                                                                                        /*
    {•$•head•:•ty•}•=>•{•Cons•<•$•head•,•Nil•>•}      MacroRuleDeclaration
      $                                               PunctuationToken
             :                                        PunctuationToken
                              <                       PunctuationToken
                                $                     PunctuationToken
                                       ,              PunctuationToken
                                             >        PunctuationToken                                                                    */
    { $ head : ty , $ ( $ tail : ty ) , * } => { Cons < $ head , HList ! ( $ ( $ tail ) , * ) > } ;                                       /*
    {•$•head•:•ty•,•$•(•$•tail•:•ty•)•,•*•}•=>•{•Cons•<•$•head•,•HList•!•(•$•(•$•tail•)•,•*•)•>•}      MacroRuleDeclaration
      $                                                                                                PunctuationToken
             :                                                                                         PunctuationToken
                  ,                                                                                    PunctuationToken
                    $                                                                                  PunctuationToken
                      (•$•tail•:•ty•)                                                                  DelimGroup
                        $                                                                              PunctuationToken
                               :                                                                       PunctuationToken
                                      ,                                                                PunctuationToken
                                        *                                                              PunctuationToken
                                                      <                                                PunctuationToken
                                                        $                                              PunctuationToken
                                                               ,                                       PunctuationToken
                                                                       !                               PunctuationToken
                                                                         (•$•(•$•tail•)•,•*•)          DelimGroup
                                                                           $                           PunctuationToken
                                                                             (•$•tail•)                DelimGroup
                                                                               $                       PunctuationToken
                                                                                        ,              PunctuationToken
                                                                                          *            PunctuationToken
                                                                                              >        PunctuationToken                   */
    { ( $ ( $ LHS : tt ) + ) } => { Expr ! ( $ ( $ LHS ) + ) } ;                                                                          /*
    {•(•$•(•$•LHS•:•tt•)•+•)•}•=>•{•Expr•!•(•$•(•$•LHS•)•+•)•}      MacroRuleDeclaration
      (•$•(•$•LHS•:•tt•)•+•)                                        DelimGroup
        $                                                           PunctuationToken
          (•$•LHS•:•tt•)                                            DelimGroup
            $                                                       PunctuationToken
                  :                                                 PunctuationToken
                         +                                          PunctuationToken
                                         !                          PunctuationToken
                                           (•$•(•$•LHS•)•+•)        DelimGroup
                                             $                      PunctuationToken
                                               (•$•LHS•)            DelimGroup
                                                 $                  PunctuationToken
                                                         +          PunctuationToken                                                      */
    { HList ! [ $ ( $ LHS : tt ) * ] + $ ( $ RHS : tt ) + } => { < Expr ! ( HList ! [ $ ( $ LHS ) * ] ) as Add < Expr ! ( $ ( $ RHS ) + ) >> :: Output } ;/*
    {•HList•!•[•$•(•$•LHS•:•tt•)•*•]•+•$•(•$•RHS•:•tt•)•+•}•=>•{•<•Expr•!•(•HList•!•[•$•(•$•LHS•)•*•]•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::•Output•}      MacroRuleDeclaration
            !                                                                                                                                                 PunctuationToken
              [•$•(•$•LHS•:•tt•)•*•]                                                                                                                          DelimGroup
                $                                                                                                                                             PunctuationToken
                  (•$•LHS•:•tt•)                                                                                                                              DelimGroup
                    $                                                                                                                                         PunctuationToken
                          :                                                                                                                                   PunctuationToken
                                 *                                                                                                                            PunctuationToken
                                     +                                                                                                                        PunctuationToken
                                       $                                                                                                                      PunctuationToken
                                         (•$•RHS•:•tt•)                                                                                                       DelimGroup
                                           $                                                                                                                  PunctuationToken
                                                 :                                                                                                            PunctuationToken
                                                        +                                                                                                     PunctuationToken
                                                                 <                                                                                            PunctuationToken
                                                                        !                                                                                     PunctuationToken
                                                                          (•HList•!•[•$•(•$•LHS•)•*•]•)                                                       DelimGroup
                                                                                  !                                                                           PunctuationToken
                                                                                    [•$•(•$•LHS•)•*•]                                                         DelimGroup
                                                                                      $                                                                       PunctuationToken
                                                                                        (•$•LHS•)                                                             DelimGroup
                                                                                          $                                                                   PunctuationToken
                                                                                                  *                                                           PunctuationToken
                                                                                                               <                                              PunctuationToken
                                                                                                                      !                                       PunctuationToken
                                                                                                                        (•$•(•$•RHS•)•+•)                     DelimGroup
                                                                                                                          $                                   PunctuationToken
                                                                                                                            (•$•RHS•)                         DelimGroup
                                                                                                                              $                               PunctuationToken
                                                                                                                                      +                       PunctuationToken
                                                                                                                                          >>                  PunctuationToken
                                                                                                                                             ::               PunctuationToken*/
    { $ LHS : tt + $ ( $ RHS : tt ) + } => { < Expr ! ( $ LHS ) as Add < Expr ! ( $ ( $ RHS ) + ) >> ::  Output } ;                       /*
    {•$•LHS•:•tt•+•$•(•$•RHS•:•tt•)•+•}•=>•{•<•Expr•!•(•$•LHS•)•as•Add•<•Expr•!•(•$•(•$•RHS•)•+•)•>>•::••Output•}      MacroRuleDeclaration
      $                                                                                                                PunctuationToken
            :                                                                                                          PunctuationToken
                 +                                                                                                     PunctuationToken
                   $                                                                                                   PunctuationToken
                     (•$•RHS•:•tt•)                                                                                    DelimGroup
                       $                                                                                               PunctuationToken
                             :                                                                                         PunctuationToken
                                    +                                                                                  PunctuationToken
                                             <                                                                         PunctuationToken
                                                    !                                                                  PunctuationToken
                                                      (•$•LHS•)                                                        DelimGroup
                                                        $                                                              PunctuationToken
                                                                       <                                               PunctuationToken
                                                                              !                                        PunctuationToken
                                                                                (•$•(•$•RHS•)•+•)                      DelimGroup
                                                                                  $                                    PunctuationToken
                                                                                    (•$•RHS•)                          DelimGroup
                                                                                      $                                PunctuationToken
                                                                                              +                        PunctuationToken
                                                                                                  >>                   PunctuationToken
                                                                                                     ::                PunctuationToken   */
    { $ LHS : ty } => { $ LHS } ;                                                                                                         /*
    {•$•LHS•:•ty•}•=>•{•$•LHS•}      MacroRuleDeclaration
      $                              PunctuationToken
            :                        PunctuationToken
                        $            PunctuationToken                                                                                     */
    ($($name:ident {                                                                                                                      /*
    ($($name:ident•{↲    <MacroRuleDeclaration>
     $($name:ident•{↲    <MacroGroup>
       $name:ident       MacroParameterDeclaration
       $name             McIdentifier
                   {↲    <DelimGroup>                                                                                                     */
        $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*                                                        /*
        $(fn•$method:ident($($arg:ident:•$arg_ty:ty),*•$(,)?)•$(->•$ret_ty:ty)*;)*    MacroGroup
             $method:ident                                                            MacroParameterDeclaration
             $method                                                                  McIdentifier
                          ($($arg:ident:•$arg_ty:ty),*•$(,)?)                         DelimGroup
                           $($arg:ident:•$arg_ty:ty),*                                MacroGroup
                             $arg:ident                                               MacroParameterDeclaration
                             $arg                                                     McIdentifier
                                       :                                              PunctuationToken
                                         $arg_ty:ty                                   MacroParameterDeclaration
                                         $arg_ty                                      McIdentifier
                                                    ,                                 PunctuationToken
                                                       $(,)?                          MacroGroup
                                                         ,                            PunctuationToken
                                                              $(->•$ret_ty:ty)*       MacroGroup
                                                                ->                    PunctuationToken
                                                                   $ret_ty:ty         MacroParameterDeclaration
                                                                   $ret_ty            McIdentifier
                                                                               ;      PunctuationToken                                    */
    }),* $(,)?) => {};                                                                                                                    /*
••••}),*•$(,)?)•=>•{}     </MacroRuleDeclaration>
••••}),*                  </MacroGroup>
••••}                     </DelimGroup>
      ,                   PunctuationToken
         $(,)?            MacroGroup
           ,              PunctuationToken                                                                                                */
}                                                                                                                                         /*
}    </MacroRulesDeclaration>                                                                                                             */

// Discarded Nodes: 0
// Parsed Nodes: 2004
// state_rollbacks: 1
// Total '.charCodeAt()' calls: 9790 (12% re-reads)
// Unnecessary 'skip_whitespace()' calls: 750
// source: "../../samples/macro/macro.match.rs"