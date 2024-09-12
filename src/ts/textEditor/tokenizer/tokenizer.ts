import { DefaultTokenizerPreset } from "./presets/defaultTokenizerPreset";
import { TokenizerPreset } from "./tokenizerPreset";

export interface Token
{
    type: 'text' | 'component' | 'delimeter' | 'number' | 'wildcard' | 'keyword' | 'unknown';
    value: string;
}

export class Tokenizer
{
    // private operators = ['+', '-', '*', '/', '=', '==', '===', '!=', '!=='];
    private tokenizerPreset: TokenizerPreset = new DefaultTokenizerPreset();
    
    /**
     * Tokenizes the input string. Uses the default {@link TokenizerPreset} if none are provided.
     */
    tokenize(input: string): Token[] 
    {
        // Implementation to tokenize the input
        // This should identify potential component tokens
        const tokens: Token[] = [];
        let current = 0;
        console.log(`Tokenizing: ${input} (${input.length})`);
        console.log(`Using preset: ${this.tokenizerPreset}`);

        while (current < input.length) {
            let char = input[current];
            
            let result = this.tokenizerPreset.match_all(input, char, current);

            if (result.token)
            {
                tokens.push(result.token);
                current += result.index_delta;
                continue;
            }
            
            // Handle unknown characters
            tokens.push({ type: 'unknown', value: char });
            current++;
        }

        console.log(tokens);

        return tokens;
    }

    setPreset(preset: TokenizerPreset)
    {
        this.tokenizerPreset = preset;
    }
}