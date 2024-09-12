import { TokenParseResult, TokenPattern } from "../tokenizerPreset";

export class UnicodeTokenPattern implements TokenPattern
{
    matchWords: boolean = true;

    match(input: string, char: string, current_idx: number): TokenParseResult
    {
        let change = 0;
        let result: TokenParseResult = { token: undefined, index_delta: 0 };
        // console.log('tokenizing unicode');
        while (current_idx < input.length)
        {
            // Check if character is an alphabetic number
            // Regex sourced from https://stackoverflow.com/a/62032796
            if (RegExp(/^\p{L}/, 'u').test(char))
            {
                current_idx++;
                change++;

                result.token = { type: 'text', value: char };
                result.index_delta = change;
                
                if (!this.matchWords) return result;

                let value = char;
                char = input[++current_idx];

                while (RegExp(/^\p{L}\s/, 'u').test(char) && current_idx < input.length)
                {
                    change++;
                    value += char;
                    char = input[++current_idx];
                }

                result.token = { type: 'text', value: char };
                result.index_delta = change;
                return result;
            }

            return result;
        }

        return result;
    }
    
}