import { TokenParseResult, TokenPattern } from "../tokenizerPreset";

export class DelimiterTokenPattern implements TokenPattern
{
    private delimiter = ',';
    private matchTrailingSpaces: boolean = true;
    match(input: string, char: string, current_idx: number): TokenParseResult
    {
        let change = 0;
        let result: TokenParseResult = { token: undefined, index_delta: 0 }
        // console.log('tokenizing delimiter');
        if (char === this.delimiter)
        {
            change += 1;
            result.token = { type: 'delimeter', value: char };
            if (!this.matchTrailingSpaces) return result;
            
            let value = char;
            current_idx += 1;
            char = input[current_idx];
            while (/\s/i.test(char))
            {
                change += 1;
                value += char;
                char = input[++current_idx];
            }

            result.token.value = value;
            result.index_delta = change;
            return result;
        }
        return result;
    }
    
    overrideDelimiter(delimiter: string)
    {
        this.delimiter = delimiter;
    }
}