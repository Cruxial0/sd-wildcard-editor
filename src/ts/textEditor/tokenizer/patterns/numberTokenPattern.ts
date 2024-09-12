import { TokenParseResult, TokenPattern } from "../tokenizerPreset";

export class NumberTokenPattern implements TokenPattern
{
    matchDecimals: boolean = true;
    decimals: string[] = ['\\.'];
    match(input: string, char: string, current_idx: number): TokenParseResult
    {
        // console.log('tokenizing decimal');
        return this.matchDecimals
            ? this.matchWithDecimals(input, char, current_idx)
            : this.matchWithoutDecimals(input, char, current_idx)
    }

    private matchWithDecimals(input: string, char: string, current_idx: number): TokenParseResult
    {
        // console.log('with decimals');
        let change = 0;
        let result: TokenParseResult = { token: undefined, index_delta: 0 }
        let decimalCount = 0;
        let regex = this.generateRegex();

        if (RegExp(regex).test(char)) {
            let value = '';
            
            while (RegExp(regex).test(char))
            {
                // Ensure a valid decimal number
                if (this.decimals.includes(char) && current_idx + 1 < input.length)
                {
                    decimalCount++;
                    // If a decimal character is not followed by  a number, or the number of decimals exceeds one,
                    // Assume invalid decimal number
                    if (!/[0-9]/.test(input[current_idx + 1]) || decimalCount > 1)
                    {
                        result.token = { type: 'number', value: value }
                        result.index_delta = change;
                        return result;
                    }
                }
                else if (this.decimals.includes(char) && current_idx + 1 > input.length)
                {
                    result.token = { type: 'number', value: value }
                    result.index_delta = change;
                    return result;
                }

                value += char;
                char = input[++current_idx];
                change++;
            }

            if (value.length > 0 && current_idx + 1 < input.length)
            {
                console.log(input[current_idx - 1] + char);
                
                // If a number is directly connected to text, it's probably part of the text.
                if (RegExp(/^\p{L}/).test(char) || /[a-z]/i.test(char))
                {
                    console.log("returning number as text");
                    
                    result.token = { type: 'text', value: value }
                    result.index_delta = change;
                    return result;
                }
            }
            
            result.token = { type: 'number', value: value }
            result.index_delta = change;
            return result;
        }
        return result;
    }

    private matchWithoutDecimals(input: string, char: string, current_idx: number): TokenParseResult
    {
        let change = 0;
        let result: TokenParseResult = { token: undefined, index_delta: 0 }
        // console.log('without decimals');
        
        if (/[0-9]/.test(char)) {
            let value = '';
            while (/[0-9]/.test(char)) {
                value += char;
                char = input[++current_idx];
                change++;
            }
            
            result.token = { type: 'number', value: value }
            result.index_delta = change;
            return result;
        }

        return result;
    }

    private generateRegex(): string
    {
        let base = "[0-9|";
        base += this.decimals.join('');
        return base + "]";
    }
    
}