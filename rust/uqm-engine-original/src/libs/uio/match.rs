/*
 * Copyright (C) 2003  Serge van den Boom
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * Nota bene: later versions of the GNU General Public License do not apply
 * to this program.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

// Rust translation of src/libs/uio/match.c.
extern "C" {
    pub type _IO_FILE;
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strstr(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn strdup(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn regcomp(
        _: *mut regex_t,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn regexec(
        _: *const regex_t,
        _: *const ::core::ffi::c_char,
        _: size_t,
        _: *mut regmatch_t,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn regfree(_: *mut regex_t);
    fn regerror(
        _: ::core::ffi::c_int,
        _: *const regex_t,
        _: *mut ::core::ffi::c_char,
        _: size_t,
    ) -> size_t;
    fn malloc(_: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
}
pub type size_t = ::core::ffi::c_ulong;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_MatchContext {
    pub r#type: match_MatchType,
    pub u: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub literal: *mut match_LiteralContext,
    pub prefix: *mut match_PrefixContext,
    pub suffix: *mut match_SuffixContext,
    pub subString: *mut match_SubStringContext,
    pub regex: *mut match_RegexContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_RegexContext {
    pub native: regex_t,
    pub errorString: *mut ::core::ffi::c_char,
    pub errorCode: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub re_nsub: size_t,
    pub __opaque: *mut ::core::ffi::c_void,
    pub __padding: [*mut ::core::ffi::c_void; 4],
    pub __nsub2: size_t,
    pub __padding2: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_SubStringContext {
    pub pattern: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_SuffixContext {
    pub pattern: *mut ::core::ffi::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_PrefixContext {
    pub pattern: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_LiteralContext {
    pub pattern: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct match_MatchType(pub ::core::ffi::c_uint);
impl match_MatchType {
    pub const match_MATCH_LITERAL: Self = Self(0);
    pub const match_MATCH_PREFIX: Self = Self(1);
    pub const match_MATCH_SUFFIX: Self = Self(2);
    pub const match_MATCH_SUBSTRING: Self = Self(3);
    pub const match_MATCH_REGEX: Self = Self(4);
}
pub type match_Result = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type regoff_t = isize;
#[inline]
unsafe extern "C" fn match_allocMatchContext() -> *mut match_MatchContext {
    return malloc(::core::mem::size_of::<match_MatchContext>() as size_t)
        as *mut match_MatchContext;
}
#[inline]
unsafe extern "C" fn match_freeMatchContext(mut context: *mut match_MatchContext) {
    free(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn match_prepareContext(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_MatchContext,
    mut r#type: match_MatchType,
) -> match_Result {
    let mut result: match_Result = 0;
    *contextPtr = match_allocMatchContext();
    (**contextPtr).r#type = r#type;
    match r#type {
        match_MatchType::match_MATCH_LITERAL => {
            result = match_prepareLiteral(pattern, &raw mut (**contextPtr).u.literal);
        }
        match_MatchType::match_MATCH_PREFIX => {
            result = match_preparePrefix(pattern, &raw mut (**contextPtr).u.prefix);
        }
        match_MatchType::match_MATCH_SUFFIX => {
            result = match_prepareSuffix(pattern, &raw mut (**contextPtr).u.suffix);
        }
        match_MatchType::match_MATCH_SUBSTRING => {
            result = match_prepareSubString(
                pattern,
                &raw mut (**contextPtr).u.subString,
            );
        }
        match_MatchType::match_MATCH_REGEX => {
            result = match_prepareRegex(pattern, &raw mut (**contextPtr).u.regex);
        }
        _ => {
            fprintf(
                stderr,
                b"match_prepareContext called with unsupported type %d matching.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                r#type.0,
            );
            return -2 as match_Result;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchPattern(
    mut context: *mut match_MatchContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    match (*context).r#type {
        match_MatchType::match_MATCH_LITERAL => {
            return match_matchLiteral((*context).u.literal, string);
        }
        match_MatchType::match_MATCH_PREFIX => {
            return match_matchPrefix((*context).u.prefix, string);
        }
        match_MatchType::match_MATCH_SUFFIX => {
            return match_matchSuffix((*context).u.suffix, string);
        }
        match_MatchType::match_MATCH_SUBSTRING => {
            return match_matchSubString((*context).u.subString, string);
        }
        match_MatchType::match_MATCH_REGEX => {
            return match_matchRegex((*context).u.regex, string);
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_errorString(
    mut context: *mut match_MatchContext,
    mut result: match_Result,
) -> *const ::core::ffi::c_char {
    match result {
        0 | 1 => return b"Success\0".as_ptr() as *const ::core::ffi::c_char,
        -2 => return b"Not implemented\0".as_ptr() as *const ::core::ffi::c_char,
        -4 => return b"Uninitialised use\0".as_ptr() as *const ::core::ffi::c_char,
        -3 => {}
        _ => return b"Unknown error\0".as_ptr() as *const ::core::ffi::c_char,
    }
    if context.is_null() {
        return b"Unknown match-type specific error.\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    match (*context).r#type {
        match_MatchType::match_MATCH_REGEX => {
            return match_errorStringRegex((*context).u.regex, result);
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_freeContext(mut context: *mut match_MatchContext) {
    match (*context).r#type {
        match_MatchType::match_MATCH_LITERAL => {
            match_freeLiteral((*context).u.literal);
        }
        match_MatchType::match_MATCH_PREFIX => {
            match_freePrefix((*context).u.prefix);
        }
        match_MatchType::match_MATCH_SUFFIX => {
            match_freeSuffix((*context).u.suffix);
        }
        match_MatchType::match_MATCH_SUBSTRING => {
            match_freeSubString((*context).u.subString);
        }
        match_MatchType::match_MATCH_REGEX => {
            match_freeRegex((*context).u.regex);
        }
        _ => {
            abort();
        }
    }
    match_freeMatchContext(context);
}
#[no_mangle]
pub unsafe extern "C" fn match_matchPatternOnce(
    mut pattern: *const ::core::ffi::c_char,
    mut r#type: match_MatchType,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    let mut context: *mut match_MatchContext = ::core::ptr::null_mut::<
        match_MatchContext,
    >();
    let mut result: match_Result = 0;
    result = match_prepareContext(pattern, &raw mut context, r#type);
    if result == 0 as ::core::ffi::c_int {
        result = match_matchPattern(context, string);
    }
    match_freeContext(context);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn match_prepareLiteral(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_LiteralContext,
) -> match_Result {
    *contextPtr = match_newLiteralContext(strdup(pattern));
    return 0 as match_Result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchLiteral(
    mut context: *mut match_LiteralContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    return if strcmp((*context).pattern, string) == 0 as ::core::ffi::c_int {
        1 as match_Result
    } else {
        0 as match_Result
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_freeLiteral(mut context: *mut match_LiteralContext) {
    free((*context).pattern as *mut ::core::ffi::c_void);
    match_freeLiteralContext(context);
}
#[inline]
unsafe extern "C" fn match_newLiteralContext(
    mut pattern: *mut ::core::ffi::c_char,
) -> *mut match_LiteralContext {
    let mut result: *mut match_LiteralContext = ::core::ptr::null_mut::<
        match_LiteralContext,
    >();
    result = match_allocLiteralContext();
    (*result).pattern = pattern;
    return result;
}
#[inline]
unsafe extern "C" fn match_allocLiteralContext() -> *mut match_LiteralContext {
    return malloc(::core::mem::size_of::<match_LiteralContext>() as size_t)
        as *mut match_LiteralContext;
}
#[inline]
unsafe extern "C" fn match_freeLiteralContext(mut context: *mut match_LiteralContext) {
    free(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn match_preparePrefix(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_PrefixContext,
) -> match_Result {
    *contextPtr = match_newPrefixContext(strdup(pattern));
    return 0 as match_Result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchPrefix(
    mut context: *mut match_PrefixContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    let mut patPtr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    patPtr = (*context).pattern;
    loop {
        if *patPtr as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return 1 as match_Result;
        }
        if *string as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            return 0 as match_Result;
        }
        if *patPtr as ::core::ffi::c_int != *string as ::core::ffi::c_int {
            return 0 as match_Result;
        }
        patPtr = patPtr.offset(1);
        string = string.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_freePrefix(mut context: *mut match_PrefixContext) {
    free((*context).pattern as *mut ::core::ffi::c_void);
    match_freePrefixContext(context);
}
#[inline]
unsafe extern "C" fn match_newPrefixContext(
    mut pattern: *mut ::core::ffi::c_char,
) -> *mut match_PrefixContext {
    let mut result: *mut match_PrefixContext = ::core::ptr::null_mut::<
        match_PrefixContext,
    >();
    result = match_allocPrefixContext();
    (*result).pattern = pattern;
    return result;
}
#[inline]
unsafe extern "C" fn match_allocPrefixContext() -> *mut match_PrefixContext {
    return malloc(::core::mem::size_of::<match_PrefixContext>() as size_t)
        as *mut match_PrefixContext;
}
#[inline]
unsafe extern "C" fn match_freePrefixContext(mut context: *mut match_PrefixContext) {
    free(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn match_prepareSuffix(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_SuffixContext,
) -> match_Result {
    *contextPtr = match_newSuffixContext(strdup(pattern), strlen(pattern));
    return 0 as match_Result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchSuffix(
    mut context: *mut match_SuffixContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    let mut stringLen: size_t = 0;
    stringLen = strlen(string) as size_t;
    if stringLen < (*context).len {
        return 0 as match_Result;
    }
    return if memcmp(
        string.offset(stringLen as isize).offset(-((*context).len as isize))
            as *const ::core::ffi::c_void,
        (*context).pattern as *const ::core::ffi::c_void,
        (*context).len,
    ) == 0 as ::core::ffi::c_int
    {
        1 as match_Result
    } else {
        0 as match_Result
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_freeSuffix(mut context: *mut match_SuffixContext) {
    free((*context).pattern as *mut ::core::ffi::c_void);
    match_freeSuffixContext(context);
}
#[inline]
unsafe extern "C" fn match_newSuffixContext(
    mut pattern: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> *mut match_SuffixContext {
    let mut result: *mut match_SuffixContext = ::core::ptr::null_mut::<
        match_SuffixContext,
    >();
    result = match_allocSuffixContext();
    (*result).pattern = pattern;
    (*result).len = len;
    return result;
}
#[inline]
unsafe extern "C" fn match_allocSuffixContext() -> *mut match_SuffixContext {
    return malloc(::core::mem::size_of::<match_SuffixContext>() as size_t)
        as *mut match_SuffixContext;
}
#[inline]
unsafe extern "C" fn match_freeSuffixContext(mut context: *mut match_SuffixContext) {
    free(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn match_prepareSubString(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_SubStringContext,
) -> match_Result {
    *contextPtr = match_newSubStringContext(strdup(pattern));
    return 0 as match_Result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchSubString(
    mut context: *mut match_SubStringContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    return !strstr(string, (*context).pattern).is_null() as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn match_freeSubString(mut context: *mut match_SubStringContext) {
    free((*context).pattern as *mut ::core::ffi::c_void);
    match_freeSubStringContext(context);
}
#[inline]
unsafe extern "C" fn match_newSubStringContext(
    mut pattern: *mut ::core::ffi::c_char,
) -> *mut match_SubStringContext {
    let mut result: *mut match_SubStringContext = ::core::ptr::null_mut::<
        match_SubStringContext,
    >();
    result = match_allocSubStringContext();
    (*result).pattern = pattern;
    return result;
}
#[inline]
unsafe extern "C" fn match_allocSubStringContext() -> *mut match_SubStringContext {
    return malloc(::core::mem::size_of::<match_SubStringContext>() as size_t)
        as *mut match_SubStringContext;
}
#[inline]
unsafe extern "C" fn match_freeSubStringContext(
    mut context: *mut match_SubStringContext,
) {
    free(context as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn match_prepareRegex(
    mut pattern: *const ::core::ffi::c_char,
    mut contextPtr: *mut *mut match_RegexContext,
) -> match_Result {
    *contextPtr = match_newRegexContext();
    (**contextPtr).errorCode = regcomp(
        &raw mut (**contextPtr).native,
        pattern,
        1 as ::core::ffi::c_int | 8 as ::core::ffi::c_int,
    );
    if (**contextPtr).errorCode == 0 as ::core::ffi::c_int {
        (**contextPtr).flags = 1 as ::core::ffi::c_int;
        return 0 as match_Result;
    }
    return -3 as match_Result;
}
#[no_mangle]
pub unsafe extern "C" fn match_matchRegex(
    mut context: *mut match_RegexContext,
    mut string: *const ::core::ffi::c_char,
) -> match_Result {
    let mut retval: ::core::ffi::c_int = 0;
    if (*context).flags & 1 as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        return -4 as match_Result;
    }
    if !(*context).errorString.is_null() {
        free((*context).errorString as *mut ::core::ffi::c_void);
        (*context).errorString = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    retval = regexec(
        &raw mut (*context).native,
        string,
        0 as size_t,
        ::core::ptr::null_mut::<regmatch_t>(),
        0 as ::core::ffi::c_int,
    );
    match retval {
        0 => return 1 as match_Result,
        1 => return 0 as match_Result,
        _ => {
            (*context).errorCode = retval;
            return -3 as match_Result;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn match_errorStringRegex(
    mut context: *mut match_RegexContext,
    mut errorCode: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut errorStringLength: size_t = 0;
    if !(*context).errorString.is_null() {
        free((*context).errorString as *mut ::core::ffi::c_void);
    }
    errorStringLength = regerror(
        (*context).errorCode,
        &raw mut (*context).native,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as size_t,
    );
    (*context).errorString = malloc(errorStringLength) as *mut ::core::ffi::c_char;
    regerror(
        (*context).errorCode,
        &raw mut (*context).native,
        (*context).errorString,
        errorStringLength,
    );
    return (*context).errorString;
}
#[no_mangle]
pub unsafe extern "C" fn match_freeRegex(mut context: *mut match_RegexContext) {
    regfree(&raw mut (*context).native);
    if !(*context).errorString.is_null() {
        free((*context).errorString as *mut ::core::ffi::c_void);
    }
    match_freeRegexContext(context);
}
#[inline]
unsafe extern "C" fn match_newRegexContext() -> *mut match_RegexContext {
    let mut result: *mut match_RegexContext = ::core::ptr::null_mut::<
        match_RegexContext,
    >();
    result = match_allocRegexContext();
    (*result).errorString = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*result).errorCode = 0 as ::core::ffi::c_int;
    (*result).flags = 0 as ::core::ffi::c_int;
    return result;
}
#[inline]
unsafe extern "C" fn match_allocRegexContext() -> *mut match_RegexContext {
    return malloc(::core::mem::size_of::<match_RegexContext>() as size_t)
        as *mut match_RegexContext;
}
#[inline]
unsafe extern "C" fn match_freeRegexContext(mut context: *mut match_RegexContext) {
    free(context as *mut ::core::ffi::c_void);
}
