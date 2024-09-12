import { Token } from "./tokenizer"

export interface TokenizerPresetParameters
{
    tokenPatterns: TokenPattern[];
}

export abstract class TokenizerPreset implements TokenizerPresetParameters
{
    /** 
    The patterns to execute. Order matters!
    */
    abstract tokenPatterns: TokenPattern[];
    match_all(input: string, char: string, current_idx: number): TokenParseResult
    {
        for (var i = 0; i < this.tokenPatterns.length; i++)
        {
            let result = this.tokenPatterns[i].match(input, char, current_idx);
            if (result.token) return result;
        }

        return {token: undefined, index_delta: 0}
    }
    
}

export interface TokenPattern
{
    match(input: string, char: string, current_idx: number): TokenParseResult
}

export interface TokenParseResult
{
    token: Token | undefined;
    index_delta: number;
}