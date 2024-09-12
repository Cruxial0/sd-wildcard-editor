import { TokenParseResult, TokenPattern } from "../tokenizerPreset";

export class TextTokenPattern implements TokenPattern
{
    private keywords = ['function', 'const', 'let', 'var', 'if', 'else', 'for', 'while', 'return'];

    match(input: string, char: string, current_idx: number): TokenParseResult
    {
        let change = 0;
        let result: TokenParseResult = { token: undefined, index_delta: 0 }
        // console.log('tokenizing text');
        while (current_idx < input.length)
        {
            if (/[a-z]/i.test(char)) {
                let value = '';
                while (/[a-z0-9\s-]/i.test(char) && current_idx < input.length)
                {
                    value += char;
                    char = input[++current_idx];
                    change++;
                }
                const type = this.keywords.includes(value) ? 'keyword' : 'text';
                result.token = { type, value };
                result.index_delta = change;
            }

            return result;
        }
        return result;
    }
    
}