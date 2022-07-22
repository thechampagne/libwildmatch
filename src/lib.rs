/*
* Copyright (c) 2022 XXIV
* 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
* 
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
* 
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use wildmatch::WildMatch;

/// Match strings against a simple wildcard pattern. Tests a wildcard pattern p against an input string s.
/// Returns true only when p matches the entirety of s.
///
/// No escape characters are defined.
///
/// ? matches exactly one occurrence of any character.
/// * matches arbitrary many (including zero) occurrences of any character.
/// 
/// Example:
/// * *
/// int main()
/// {
///   assert(wildmatch_matches("cat", "cat") != 0);
///   assert(wildmatch_matches("*cat*", "dog_cat_dog") != 0);
///   assert(wildmatch_matches("c?t", "cat") != 0);
///   assert(wildmatch_matches("c?t", "cot") != 0);
///   assert(wildmatch_matches("dog", "cat") == 0);
///   assert(wildmatch_matches("*d", "cat") == 0);
///   assert(wildmatch_matches("????", "cat") == 0);
///   assert(wildmatch_matches("?", "cat") == 0);
///   return 0;
/// }
/// * *
/// 
/// @param pattern
/// @param input
/// @return 0 on success and non zero value on failure
#[no_mangle]
unsafe extern "C" fn wildmatch_matches(pattern: *const c_char, input: *const c_char) -> c_int {
  if pattern.is_null() || input.is_null() {
    return 0
  }
  let p_rs = match CStr::from_ptr(pattern).to_str() {
      Ok(s) => s,
      Err(_) => return 0,
  };
  let i_rs = match CStr::from_ptr(input).to_str() {
      Ok(s) => s,
      Err(_) => return 0,
  };

  if WildMatch::new(p_rs).matches(i_rs) {
    1
  } else {
    0
  }
}