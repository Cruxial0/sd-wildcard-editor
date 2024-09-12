import { DelimiterTokenPattern } from "../patterns/delimiterTokenPattern";
import { NumberTokenPattern } from "../patterns/numberTokenPattern";
import { TextTokenPattern } from "../patterns/textTokenPattern.";
import { UnicodeTokenPattern } from "../patterns/unicodeTokenPattern";
import { WildcardTokenPattern } from "../patterns/wildcardTokenPattern";
import { TokenPattern, TokenizerPreset } from "../tokenizerPreset";

export class DefaultTokenizerPreset extends TokenizerPreset
{
    tokenPatterns: TokenPattern[] = [
        new TextTokenPattern(),
        new NumberTokenPattern(),
        new WildcardTokenPattern(),
        new DelimiterTokenPattern(),
        new UnicodeTokenPattern()
    ];
}