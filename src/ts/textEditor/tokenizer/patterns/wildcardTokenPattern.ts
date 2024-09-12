import { TokenParseResult, TokenPattern } from "../tokenizerPreset";

export class WildcardTokenPattern implements TokenPattern
{
    match(input: string, char: string, current_idx: number): TokenParseResult
    {
        
        let result: TokenParseResult = {token: undefined, index_delta: 0}
        while (current_idx < input.length)
        {
            if (/_/.test(char) && /_/.test(input[current_idx + 1]))
            {
                let value = '__';
                // Keep track of index change in case we need to revert later
                let change = 0;          

                // Skip '__' characters and set new index
                current_idx += 2;
                change += 2;
                char = input[current_idx];

                if (!char)
                {
                    return result;
                }

                while (/[a-z0-9|-]/i.test(char) || current_idx + 1 > input.length)
                {
                    value += char;
                    if (input.length < current_idx + 1) break;
                    char = input[++current_idx];
                    change++;
                }
                
                if (current_idx + 1 < input.length)
                {
                    console.log("reached end of wildcard");
                        
                    if(/_/.test(char) && /_/.test(input[current_idx + 1]))
                    {
                        while (/_/.test(char) || current_idx > input.length)
                        {
                            value += char;
                            char = input[++current_idx];
                            change++;
                        }
                        
                        const type = 'wildcard';
                        result.token = { type, value };
                        result.index_delta = change;
                        console.log("returning result:");
                        console.log(result);
                        
                        return result;
                    }
                }
    
                result.token = undefined;
                result.index_delta = 0;
                return result;
            }

            // outer while loop
            return result;
        }
        return result;
    }
    
}