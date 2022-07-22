#ifndef __WILDMATCH_H__
#define __WILDMATCH_H__

#ifdef __cplusplus
extern "C" {
#endif

/*
* Match strings against a simple wildcard pattern. Tests a wildcard pattern p against an input string s.
* Returns true only when p matches the entirety of s.
*
* No escape characters are defined.
*
* ? matches exactly one occurrence of any character.
* * matches arbitrary many (including zero) occurrences of any character.
* 
* Example:
* * *
* int main()
* {
*   assert(wildmatch_matches("cat", "cat") != 0);
*   assert(wildmatch_matches("*cat*", "dog_cat_dog") != 0);
*   assert(wildmatch_matches("c?t", "cat") != 0);
*   assert(wildmatch_matches("c?t", "cot") != 0);
*   assert(wildmatch_matches("dog", "cat") == 0);
*   assert(wildmatch_matches("*d", "cat") == 0);
*   assert(wildmatch_matches("????", "cat") == 0);
*   assert(wildmatch_matches("?", "cat") == 0);
*   return 0;
* }
* * *
* 
* @param pattern
* @param input
* @return 0 on success and non zero value on failure
*/
extern int wildmatch_matches(const char* pattern, const char* input);

#ifdef __cplusplus
}
#endif

#endif // __WILDMATCH_H__