export interface Token {
    type: 'text' | 'component' | 'wildcard' | 'text' | 'keyword' | 'delimeter' | 'unknown';
    value: string;
}

export class Tokenizer
{
    private keywords = ['function', 'const', 'let', 'var', 'if', 'else', 'for', 'while', 'return'];
    // private operators = ['+', '-', '*', '/', '=', '==', '===', '!=', '!=='];
    private delimiter = ',';
    tokenize(input: string): Token[] 
    {
        // Implementation to tokenize the input
        // This should identify potential component tokens
        const tokens: Token[] = [];
        let current = 0;
        console.log(`Tokenizing: ${input} (${input.length})`)

        while (current < input.length) {
            let char = input[current];
            
            // // Handle whitespace
            // if (/\s/.test(char)) {
            //     let value = '';
            //     while (/\s/.test(char)) {
            //         value += char;
            //         char = input[++current];
            //     }
            //     tokens.push({ type: 'whitespace', value });
            //     continue;
            // }

            // Handle keywords and identifiers
            if (/[a-z\s]/i.test(char)) {
                let value = '';
                while (/[a-z0-9\s]/i.test(char) && current < input.length)
                {
                    console.log(`${char} - ${current}`);
                    value += char;
                    char = input[++current];
                }
                const type = this.keywords.includes(value) ? 'keyword' : 'text';
                tokens.push({ type, value });
                continue;
            }

            // Handle numbers
            if (/[0-9]/.test(char)) {
                let value = '';
                while (/[0-9]/.test(char)) {
                    value += char;
                    char = input[++current];
                }
                tokens.push({ type: 'text', value });
                continue;
            }
                
            // Handle wildcards
            if (/_/.test(char) && /_/.test(input[current + 1]))
            {
                let value = '__';
                // Keep track of index change in case we need to revert later
                let change = 0; 

                // Skip '__' characters and set new index __wilcrad__
                current += 2;
                change += 2;
                char = input[current];
                
                while (/[a-z0-9-]/i.test(char) || current > input.length)
                {
                    
                    value += char;
                    char = input[++current];
                    change++;
                }
                console.log(char);
                console.log("current; " + current + " length; " + input.length);
                if (current + 1 < input.length)
                {
                    console.log(input[current] + input[current + 1]);
                    if(/_/.test(input[current]) && /_/.test(input[current + 1]))
                    {
                        while (/_/.test(char) || current > input.length)
                        {
                            value += char;
                            char = input[++current];
                            change++;
                        }
        
                        const type = 'wildcard';
                        tokens.push({ type, value });
                        continue;
                    }
                }

                current -= change
                console.log("revert");
                char = input[current];
                continue;
            }

            // Check if character is an alphabetic number
            // Regex sourced from https://stackoverflow.com/a/62032796
            if (RegExp(/^\p{L}/, 'u').test(char))
            {
                tokens.push({ type: 'text', value: char });
                current++;
                continue;
            }

            if (char === this.delimiter)
            {
                tokens.push({ type: 'delimeter', value: char });
                current++
                continue;
            }
            
            // Handle unknown characters
            tokens.push({ type: 'unknown', value: char });
            current++;
        }

        return tokens;
    }
}