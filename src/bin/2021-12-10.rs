fn main() {
    let commands = [
        // "[({(<(())[]>[[{[]{<()<>>",
        // "[(()[<>])]({[<{<<[]>>(",
        // "{([(<{}[<>[]}>{[]{[(<()>",
        // "(((({<>}<{<{<>}{[]{[]{}",
        // "[[<[([]))<([[{}[[()]]]",
        // "[{[{({}]{}}([{[{{{}}([]",
        // "{<[[]]>}<{[{[{[]{()[[[]",
        // "[<(<(<(<{}))><([]([]()",
        // "<{([([[(<>()){}]>(<<{{",
        // "<{([{{}}[<[[[<>{}]]]>[]]",
        "([<{(<{(({<{{{[]<>}<<>()>}<<()()>[()()])}{({()[]}[<>{}])<(<><>)<[]{}>>}>{{((()<>)<[]()>)[{[]<>}(()[])]",
        "<<[[<[(<[<[[[{()<>}(<>{})]]][{(<{}{}>(<>))[[<>](<><>)]}[({{}<>}<[]()>){<{}{}>[{}[]]}]]><{(<([][]",
        "{<([[[({{(<[([{}<>]<{}{}>)[[<>[]]]]{{(()<>)[()]}}>){((([{}<>]{[]{}})<<[][]><<>{}>>){([[]{}]<<>()>){",
        "{{[{({{(({<[<{[]{}](<>)>][<(()[])(()[])>[[<>{}](<>())]]><<([{}]{{}{}})[{[]}{{}[]}]>[{<()<>><<>>}]>",
        "(<<<[(({[[[{<[{}<>]{<><>}><({})>}{{{()}{(){}}}}][{[({}{})[<><>]]<({}[])[()()]>}(<([]<>)<[](",
        "<[{[((<<[[<[({[]()}{(){}})[{<>[]}<()()>]]>]]<[{{[[[][]]<[]()>][<<>{}><<><>>]}}({[<[]()><{}[]>]{<[]",
        "(({((({{[[(([<[]<>>(())]{{()<>}<<>[]>}))]]([<([{{}()}(<>{})][({}<>)])>](({{<()<>>[{}())}{((){}){[]()}}})",
        "({{({(<<{<<([<{}>])(<[()<>][{}[])>{[()()][{}{}]})>[[[{{}[]}[(){}]][{<>{}}{()<>}]]({<<>[]>{{",
        "<[<<{[(<{[{({({}[])(()[])}[{{}<>}{{}<>}]){<{<>[]}[<>()]>{<{}()>}}}{{{{()()}}[{{}<>}({}<>)]}}}}>{{[[{<[",
        "[{([<{{<<<[([<[]()>])]([{{<>{}}}{(()[])[<>[]]}])><[(<{()<>}{<>}>[([]{})<<>{}>]}([{()<>}({}[])]<<{}[]>({}{}",
        "<([<([{{<<{<(([]<>)<{}{}>)>({[(){}]}((<>[])<{}[]>))}<{<(()[])<[]>>({[][]}(()[]))}(<{()[]}{[]()}})>>[([({()[",
        "{{{[({(<[{[[{{()<>}[<>()]}{({}<>)<{}()>}]{{[()()][(){}]}[{<>()}]}]}]>)(({<<(({{}<>}([]<>))({<>[]}<[]{",
        "({{({<([{[{{[[[]()><{}[]>]({{}[]}[{}()])}}<{[{[]()}[<>]]<[{}<>]<()<>>>}>]{<<(((){}){<>()})[[[][]]]>[([<><>]{",
        "(<[[{{[(([{{<((){}){<>{}}>{<{}[]><<>[]>}}(({<>[]}<<><>>)({()[]}<()<>>))}<{(<()()>[{}{}]){{",
        "{{{[{<<(<<[{({(){}}[{}{}])}[{[<>[]]{()()}}{[()]{(){}}}]]>>)><{((<<{<[]{}>([]{})}{{{}{}}}><([{}[]](()()))>",
        "[<(<<<[(([((<<<><>>>[<[]<>>(()[])])[{((){}){<><>}}<<{}{}>(()[])>]){(([[]<>]<[]>)<{[]}[[]<>]>)<<{{}{}}[[",
        "<(<<({(([[({[[[])(()[])][<[]()>{{}[]}]}{({{}}{(){}})(((){})([]<>))})([[([][])<{}<>>]<<()><<>()>>](",
        "[{[[<{<<{<{<{<()()><{}[]>}([(){}](<>()))>(<{<>[]}([])><[[][]](<>{})>)}(({<[]<>>{()()}}<<[][]>>){({()[]",
        "<([<{((({({[<(()[])[{}]>({[]()})]<<[[]]>>}<[[{[]{}}[{}{}]]([{}[]]{(){}})]({(<>())})>)}({<[{{{}",
        "({{([(<<{({(<(<><>)<{}<>>><<{}()>[()()]>){<{{}()}(<>[]>>}})}>>[<<<{(({{}<>}(()[])){<[][]><{}>})",
        "[<((<<{{<[[[[{[]()}][{[]{}}(<>())]]]]{<<{(()<>)((){})}<[<>[]]{[][]}>>(<(()())(()[])><{{}<>}({}())>)>[<<[(",
        "[(<<(<<[(([{((()())<<><>>}[{()<>}({}<>)]}]{{<(<>[]){(){}}>([<>()]({}{}))}})<<(<[[][]][[][]]>[([][]){(){",
        "<[([[<<[{(<([([]{}){{}<>}][{()}([][])])>)({<({(){}}){[[]{}][()()]}>{{{[]()}({}{})}(<[]{}><<>()>)}}[{[<[]()><",
        "{<{<[[{({<[<<({})<()[]>>((<>{})<[]{}>)>]>})}((<[[(<<{}{}>{[][]}>)<<[()[]]{<>{}}>{{()[]}[()]}>]<({[",
        "[({({(<<<<<[[<[]{}>]{<<>{}><[]{}>}]><<<([]{})<{}[]>><[{}[]][()<>]>>[[<{}[]>{[]}](<{}()>[{}<>])]>>>>{<{([((",
        "<({[({<{[<({{<()[]]}[{()[]}[{}<>]]}{[{()<>}{<><>}]([[][]]<[]<>>)})>[[[(<()<>>({}[]))<[(){}]<<><>>>]][[<({}{}",
        "{<(<<(([[{(<[[()[]]<()[]>]{<()<>><()[]>}><{<[]{}>{()()}}>)([([(){}]{<>{}})[{<>{}}[<>{}]]]<<[[]{}]{()",
        "{[([[{[<[<{(<<[]{}]>({<><>}({})))([{{}<>}]<{()<>}[{}[]]>)}<([(()[])<()()>])<{{[]{}}[{}()]}(<<><>>[()()]",
        "<<[{<[({((<<({{}()}{()[]})>[<<{}{}>{<><>}>]>){{<[{[]()}[[]()]]{[{}[]]}>[{<{}{}>{[]<>}}([{}[]]((){}))]}(",
        "<<{((([[({<{[(()<>)<()[]>]({()[]}({}<>))}{{{[]<>}[{}[]]}[{<><>}]}>}<[[([{}()]{(){}})<({}<>)[[]()",
        "{(([({{(([<{{{<>()}<<>()>}[{<>()}(()())]}>]{(<([()<>]{()<>}){{()<>}}><<{{}}{[][]}>(<(){}><{",
        "{<(({{[<([(<<([][])(<>()]>{{{}{}}{[]<>}}>)[{(<()[]>{<>{}})[{()[]}<[]<>>]}{<{{}[]}><<{}>[{}<>",
        "(<{{([<[[{{[{[[]()][()()]}{<<>{}>}][{{<>()}<()()>}(((){}){<>{}})]}{<(({}()))<[(){}]<(){}>>>(<({}[]>[[][]]>)}}",
        "(({({[({<{{{[[()()]({}())]}[([{}]<{}()>)([{}()]{{}{}})]}[((<{}<>><()()>))]}[{{{(<>{}){<><>)}[<()()",
        "(<({{<([{[([[({}<>)][([]<>){[]{}]]][[<()[]>]])][<{[[{}[]](())]<<()[]>(())>}{<{{}[]}[<>{}]>{<[]<>>",
        "(([[([([<[{[(({}[])<[]{}>)]}][[{({()<>}[{}[]])(([]())(()<>))}<[<[]{}>({}<>)]{{{}{}}{[][]}}>]{{{",
        "<[{[{<<{[[<{(({})(<>{}]){([]())[[]()]}}{(<()()>([]())){([]{})<[][]>}}>]{({[[<><>]{[]()}](<{}[]><[]<>>)}",
        "([{<({{([([[{{()[]}}({(){}}[<>{}])>]({{(())([]<>)}<[()<>][[]()]>}([{()[]}{()()}])))](([[[{<>{}}[<>(",
        "<[[[(<[{{[[{{{<>[]><<>>}[{()[]}({}())]}[<[{}<>]<[][]>>]]]}{{({{{<>()}{()<>}}<<{}[]>({}[])>}[[(<><",
        "[[<[<({<[({{[[<>{}][[]{}]]{<()()>{<>{}}}}[<<<>{}><[]<>>><<{}<>>(()[])>]}){<[(([]())[<>()))[({}[])<",
        "<<(<<<<((([[<<{}[]>(<>())>]]<{{[[]<>]<[]>}{[{}()]{<>[]}}}<[[<>{}]({}())]>>)){{([(({}[])({}[]",
        "([{<([<[[[{[[{<>[]}{[]()}>([[]])]<({{}{}}({}[])){(<>{})<()()>}>}<{(([]()){()<>})[{<>{}}{<>()}]}<(([]",
        "<{[<(({([[((((<>[])[[]{}])({[][]}))(<[()<>]{<><>}>)){<<{<>[]}<<>()}>>{({{}{}}(()()))}}]][{",
        "({<{<<<{{{(((([]())[<>]){<(){}><{}[]>})<{([]<>)[<><>]}[{()<>}({}<>)]>){((<[]()>[<>{}]){<()<>>(",
        "[{<(([(<[((([[(){}]{[]()}])))]>)[<{(<<[<<>[]><()()>]>>){[<{{(){}}({}[])}([{}()][<>])><((()<>)[[]])[[{}()](<>",
        "{<<[<{[(<{<([{()[]}({}<>)]{[{}<>][{}()]})([{<>{}}<<>>])>{(<{{}())<{}()>>(({}[])<<>()>))[({<>()}[()[]])<<{}<>",
        "[<([<<<[({([<<(){}><{}[]>>[[<>{}][{}<>]]]<[<()[]><(){}>]([<>{}][<>[]])>)<<{<[]<>>[[]{}]}<({}(",
        "<[[(<[{[[({{{<<>[]][{}[]]}[[[]{}]]}}){{[[(<>{})(()<>)]({<>()})]({({}{})<[]{}>}[<()<>>])}{<{[()[]]}[<",
        "<(<(<((<[((([[<>()]]){<[<>[]]><([]{})>})[<<{<>()}{{}<>}>(((){})[<>[]])>{[[<>()][{}[]]](([][])([]))}])((",
        "[(<{{<(((<<{[[<>[]]]{([]<>)<{}[]>}}[<{{}{}}[{}()]><{[]()}[{}]>]>{[<{{}[]}([]<>)>[[(){}][()()]]]",
        "<(<(([[{[<<<(({}))[{[][]}]>[[([]{})({}[])]]>[<<(<>[]){<>[]}><[()<>]>)<[{{}<>}(<>{})]{[<>()]{{}{}}}>]>{[{<{",
        "({<({{{[[(([<<()[]><<><>>>])(<((<>()))(<<><>><[]<>>)>)){{({[[]()]{[]<>}}[[{}[]]<[]()>])}({({()()}<[]()",
        "{{{<[<{<{[<(<{[]()}><[()[]]{[][]}>){<<[]>[<><>]>([<>()][<>()])}>)<<<(([]())[<>[]]){[[][]](<>{})}><([(){}]{{}{",
        "<[((<<<({<[((<{}<>>[[]{}})((<>[])))({<()()>{<><>}}{(<><>){(){}}})][[<<[]>>[{<>[]}{[]()}]]([{",
        "(((<<{<({<[[[{()()}(()<>))(<()[]>((){}))][{({}[])[{}()]}<<{}{}>{()()}>]]>})<<{{<[[<>{}]<[][]>]((<><",
        "[<[{[[{{<{[(<<<>[]><{}()>>)({[{}[]]}((<>())<()[]]))]{{({()[]}([]<>)){{<>[]}[[][]]}}}}{({(<<>[]>{[]})[[{}",
        "<<[({[[[[([<(<<>[]>{()[]})>{<[()<>]>[[[]<>]({})]}]<(<{{}}([])>[<{}()>])[{{<>()}<(){}>}<<<>{}}[{}{}]>]",
        "([(((([[<[{({<{}>})<([{}<>][(){}])<<[]()><()[]>>>]{[([<>[]](()()))[{{}[]}[{}<>]]]({({}<>)[<>()]}<{()<>}{[]{",
        "[[<{[{<{{(<(<[<>{}]><([]{})[{}<>]>)>{{[{<>[]}[{}()]]<<()>{[]}>}[[((){})([])]{<<>()>([][])}]})",
        "{<[{{{<{([<[<{{}{}}[[]{}]><{{}[]}>]{(<{}<>><{}[]>)(<(){}><()()>)}>]<<<({{}}(<>()))[<{}[]>(<>{})]>({<[]>{()<",
        "{<[{[<(({[{{({{}[]}[()[]]}}<(([]())[()[]]){(()){<>()}}>}[<<{<><>}>[([]{}){{}<>}]>[{{<>{}}{()[]}}",
        "<{[{([<{<((<[<()[]>{[][]}]<<()[]><<>{}>>><([<><>]<()()>)({{}()}<(){}>)>){<(([][]){[][]})(<[]><{}{}>)>",
        "(([<((((({{{[<()[]]{{}<>}]{<()()>({}<>)}}([[{}()]<<>>][((){})<<>[]>])}}(<{{[(){}]{()()}}}{([{}()",
        "<<<((<[[{[<([<{}<>>(<><>)]<{[]<>}<[]<>>>)>]{([{[[]<>][<>()]}({[]<>}<<>[]>)])[[{{<>[]}(()())}",
        "{<(<<<(({{(((([]<>)(<>[]))))}[(([[[][]][{}()]])<<(<>{}><{}{}>>{(()[])([]())}>)]}([({((<>){[]<>}){<[][]",
        "({(([[[[<{[{(<{}[]>[{}<>])(<[]<>>({}[]))}{([<><>]{{}{}}){{{}[]}[[][]]}}]<[((()[])([]<>))[{[][]",
        "[<([[([[<({({<[]()>}<[[]()]([]<>)>)([[()[]]]{[<>{}][()()]})))>]<({{[{<{}{}>}{{()()}[<>[]]}]{([{",
        "({<<{{({[{[{{[[]<>]<(){}>}<<<>{}>({}{})>}<{[{}<>]}(<[]>[()[]]))]}]<<<<<{(){}}<<>[]>>{<[]{}>{(){}}",
        "(({<([<((<((({{}[]}[[]()])){{[[]()]<[]{}>}(<[][]>{<>})})({{[()()]{{}<>}}(({}{})[<><>])}[[([]{})[",
        "({{[(<{{<<<<[{{}{}}{{}{}}][<[]()>[[]()]]>([{{}}<[]<>>][<[]{}>{()<>}])>[<{({}{})({}<>>}[<{}<>>[<>()]]>]><<",
        "{[((({{(<(<{[{[]()}}<<[][]>(<>[])>}<(<[]()><()<>>){[<><>]<<>()>}>><[{<()[]>}{[{}[]](<>{})}]>)<({<{<><>}>}{",
        "[(([{([<<<([[(()())][([]{})[<>{}]]]({((){})((){})}(([][])[()<>])))[<((<>())[<>{}])>{{[[]<>](",
        "<(({<([(<<<{([{}[]]<[]>)}({{<>{}}[[]{}]}(<{}[]>(()<>)))>>({<{<<>()>((){})}((()())[<>{}])><[<[]()>]{<",
        "[<[{({<[({[(<[{}[]]{[]<>}>(({}{}){{}[]}))(<{()[]}>)]{<([[][]][{}<>])<<<>{}>>>{{((){}){()()}}<{{}}",
        "[<([{<(<(<<{([[]<>]{(){}})((()<>)[[]])}<[<{}()>]<{<><>}<{}{}>>>>([{({}<>){{}()}}]<((()()))<<[][]>",
        "({[{((<{[{([<{()()}{<>()}>[[()[]]]])}[{([[{}]{<>{}}]({[]()}[{}[]]))<{<{}<>>}>}]][[[(<({}{}){{}",
        "[[<{{<{[([(<([[]<>][()<>]))[[[<>[]]]({()()}([][]))])(([[[][]]([]())])<{{{}<>}([]{})}<{[][]}{",
        "<<({<[{[<{<<([<>()]({}<>))][([{}()][{}[]])]>}>]}{[<[[[[[[]{}]<{}>]<<{}()>{()[]}>]][<{({}())",
        "(<<[({[([([[(<<>{}>){[(){}][()]}][[[(){}]{[]<>}]]]<[([<>[]]([]))[{{}<>}{()()}]]<[<{}[]>(()())]<<<><>>{{}[]",
        "{([[[<[(<({<{{{}{}}[[]<>]}<[()[]]{<>[]}>>}<<<(()())<{}[]>>[([][])<<><>>]>[(((){})(()[])){<{}()>(()[])",
        "{[<{[[[((<{[{[[]<>]{{}()}}[{{}<>}{<><>}]][<<()<>>{<>{}}>[{[]{}}((){})>]}(<[<[][]>({}())]<[<>[]](()[])",
        "{({(<<{({({([<(){}>[()()]](<[]><[]<>>))([{<>()}<{}<>>]{<[]<>>[[][]]})})({[(([]<>)[[]])]{([[]{}]{{}()}",
        "{{[<{<<{<{({(({}()))[<<>{}]{()()}]}({{[]{}}([][])}[<<>()>{[]<>}]))}>(<<[{<[]()><<>{}>}]{((<>[]))<(",
        "{{[<[{<(([({([{}{}]({}[]))<<()>(<>[])>})[<<{[][]}<[]()>><{{}[]}<<>{}>>>{{{{}{}}{<>{}}}([{}",
        "[[([(<{{[{{{{{()()}[[][]]}([{}()](<>))}<{<<>[]>[{}[]]}<<{}()>>>}}]<<[{[<{}>({})](<()[]>)}](<<<[][]>(",
        "{<[([{{{((<{((<>())){<[]<>>}}<<{<><>}[<>{}]>{{()[]}(()[])}>>{{{<()()>[[]{}]}[[<><>]<[]()>]}([{{}}{()()}][(<>[",
        "[{<{[([((<(<(((){})[<>]){({}[])<{}<>>}>(((()())([]{}))<[()()]{[]{}}>))([((()<>)<{}()})<{{}[]}<<>",
        "{(({{{<[<({([<{}()>]{<<><>>{<><>}})}){<(((<>[])<[][]>)(<[]<>>{{}<>}))(<{<>{}}<()[]>>[[{}[]]({}())])>",
        "{<([{(<{<<{(<{()}{<>[]}>)([[{}()]{<>{}}]<({}{})>)}[<<<()[]>({}<>)>[<[]>[<>{}]]><(<{}()>[<>{}])>]",
        "<{{{{(({{{[{<[[][]]<()<>>>}[({{}[]}([]<>))({{}{}}{<>()>)]]}}}([{<{{<{}<>><<><>>}}{([{}<>]<<>{}>)",
        "{<[<<{<([[[[[[<>[]](()[])]({{}[]}<{}>)][((()())({}()))<{{}[]}(<>())>]]([<[{}[]][<>[]]>]<{([]{})}{(",
        "{<{[[([<([[[(<()()>({}[]))[{()[]}]]((({}[]><()()>)[({}){[]<>}])]]{<<{<()[]>}{<<>[]>{<>()}}>><[{(()<>)({}<",
        "[[<[[<{{{{<<{{[][]}<(){}>}><{([]())[[]<>]}<<{}[]>(<>())>>>}<{{[[(){}]<<>()>]<{{}()}[[][]]>}}<(<<(",
        "<[[(({({(({{{([]{}){<>{}}}{[<><>]{[]<>}}}}))[({[[({}<>){<><>}]{<<>{}><<><>>}]}[<{<{}<>>(<>{})}(([",
        "({<<([{[<[<([((){})([]())][(()<>)({}[])]){{[{}()]({}[])}[{{}()}{{}()}]}>{{({<>[]}{<>})[<(){}>",
        "<(<{<[<{({{[<{{}[]}{[]<>)>][{[{}{}][()<>]}(<[]<>>[[]{}])]}(([[[]<>]{()()}]<[{}[]]{<>{}}>)((([][]))",
        "<{<(<<((<<[[{<()()><<><>>}<(()<>)({}<>)>>[<<[][]>(<><>)>{(<>[])}]](<{<()[]>([]())}[{<><>}<{}[]>]>)>>{(",
        "{(({(<(<<[[[{<<>()><{}()>}[[<><>]{()<>>]]{[{(){}}]}]]>>)(<<(<({([]())<<><>>}({[]()}({}<>)))>",
        "<{<<[<<{[<([[(()())<<><>>][<<>[]>(<>())]]({<{}{}><<><>>}{[{}]<<>[]>})){[{{[]}<{}[]>}[[(){}]{[]<>}]]",
        "({[[{<[([[[{[[(){}]{[][]}]<({}[])({}())>}{{<()[]>{<><>}}<{<>}>}]{<<<()<>]{<>{}}>[<[]<>>{{}()}]><[<<",
        "<{{{([({<{{([{()}[()<>]][(<>())(())])([({}[])[{}{}]]({{}{})([]())))}{(<[<>()]{()<>}><[()<>][<>[]]",
        "((<[(<[(({{(<<(){}>{(){}}>({()[]}[<>[]]))<(((){})({}()))<<[]{}><()[]>>>}})<[{([<<>{}>]){<{<>{}}{()<>}>}}{{(",
    ];

    let mut incorrect_character_score = 0;
    let mut completion_scores = vec![];
    for command in commands.iter() {
        let mut stack = vec![];
        let mut had_incorrect_characters = false;
        for character in command.chars() {
            match character {
                '(' | '[' | '{' | '<' => stack.push(character),
                ')' | ']' | '}' | '>' => {
                    let expected_character = match stack.pop().unwrap() {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => unreachable!(),
                    };
                    if character != expected_character {
                        println!(
                            "{}: Expected {}, but found {} instead.",
                            command, expected_character, character
                        );
                        incorrect_character_score += match character {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                        had_incorrect_characters = true;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
        if !had_incorrect_characters && stack.len() > 0 {
            let mut completion_score = 0_usize;
            for missing_character in stack.iter().rev() {
                completion_score *= 5;
                completion_score += match missing_character {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }
            completion_scores.push(completion_score);
            println!("Incomplete: {:?} -> score: {}", stack, completion_score);
        }
    }
    println!("Part One: {}", incorrect_character_score);
    completion_scores.sort();
    println!(
        "Part Two: {}",
        completion_scores[(completion_scores.len() / 2)]
    );
}
