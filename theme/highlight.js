/*!
  Highlight.js v11.1.0 (git: 83ad2fbd99)
  (c) 2006-2021 Ivan Sagalaev and other contributors
  License: BSD-3-Clause
 */
var hljs=function(){"use strict";var e={exports:{}};function t(e){
return e instanceof Map?e.clear=e.delete=e.set=()=>{
throw Error("map is read-only")}:e instanceof Set&&(e.add=e.clear=e.delete=()=>{
throw Error("set is read-only")
}),Object.freeze(e),Object.getOwnPropertyNames(e).forEach((n=>{var r=e[n]
;"object"!=typeof r||Object.isFrozen(r)||t(r)})),e}
e.exports=t,e.exports.default=t;var n=e.exports;class r{constructor(e){
void 0===e.data&&(e.data={}),this.data=e.data,this.isMatchIgnored=!1}
ignoreMatch(){this.isMatchIgnored=!0}}function s(e){
return e.replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#x27;")
}function i(e,...t){const n=Object.create(null);for(const t in e)n[t]=e[t]
;return t.forEach((e=>{for(const t in e)n[t]=e[t]})),n}const a=e=>!!e.kind
;class o{constructor(e,t){
this.buffer="",this.classPrefix=t.classPrefix,e.walk(this)}addText(e){
this.buffer+=s(e)}openNode(e){if(!a(e))return;let t=e.kind
;t=e.sublanguage?"language-"+t:((e,{prefix:t})=>{if(e.includes(".")){
const n=e.split(".")
;return[`${t}${n.shift()}`,...n.map(((e,t)=>`${e}${"_".repeat(t+1)}`))].join(" ")
}return`${t}${e}`})(t,{prefix:this.classPrefix}),this.span(t)}closeNode(e){
a(e)&&(this.buffer+="</span>")}value(){return this.buffer}span(e){
this.buffer+=`<span class="${e}">`}}class l{constructor(){this.rootNode={
children:[]},this.stack=[this.rootNode]}get top(){
return this.stack[this.stack.length-1]}get root(){return this.rootNode}add(e){
this.top.children.push(e)}openNode(e){const t={kind:e,children:[]}
;this.add(t),this.stack.push(t)}closeNode(){
if(this.stack.length>1)return this.stack.pop()}closeAllNodes(){
for(;this.closeNode(););}toJSON(){return JSON.stringify(this.rootNode,null,4)}
walk(e){return this.constructor._walk(e,this.rootNode)}static _walk(e,t){
return"string"==typeof t?e.addText(t):t.children&&(e.openNode(t),
t.children.forEach((t=>this._walk(e,t))),e.closeNode(t)),e}static _collapse(e){
"string"!=typeof e&&e.children&&(e.children.every((e=>"string"==typeof e))?e.children=[e.children.join("")]:e.children.forEach((e=>{
l._collapse(e)})))}}class c extends l{constructor(e){super(),this.options=e}
addKeyword(e,t){""!==e&&(this.openNode(t),this.addText(e),this.closeNode())}
addText(e){""!==e&&this.add(e)}addSublanguage(e,t){const n=e.root
;n.kind=t,n.sublanguage=!0,this.add(n)}toHTML(){
return new o(this,this.options).value()}finalize(){return!0}}function _(e){
return e?"string"==typeof e?e:e.source:null}function d(e){return g("(?:",e,")?")
}function g(...e){return e.map((e=>_(e))).join("")}function u(...e){
return"("+((e=>{const t=e[e.length-1]
;return"object"==typeof t&&t.constructor===Object?(e.splice(e.length-1,1),t):{}
})(e).capture?"":"?:")+e.map((e=>_(e))).join("|")+")"}function h(e){
return RegExp(e.toString()+"|").exec("").length-1}
const A=/\[(?:[^\\\]]|\\.)*\]|\(\??|\\([1-9][0-9]*)|\\./
;function p(e,{joinWith:t}){let n=0;return e.map((e=>{n+=1;const t=n
;let r=_(e),s="";for(;r.length>0;){const e=A.exec(r);if(!e){s+=r;break}
s+=r.substring(0,e.index),
r=r.substring(e.index+e[0].length),"\\"===e[0][0]&&e[1]?s+="\\"+(Number(e[1])+t):(s+=e[0],
"("===e[0]&&n++)}return s})).map((e=>`(${e})`)).join(t)}
const f="[a-zA-Z]\\w*",R="[a-zA-Z_]\\w*",b="\\b\\d+(\\.\\d+)?",M="(-?)(\\b0[xX][a-fA-F0-9]+|(\\b\\d+(\\.\\d*)?|\\.\\d+)([eE][-+]?\\d+)?)",E="\\b(0b[01]+)",m={
begin:"\\\\[\\s\\S]",relevance:0},T={scope:"string",begin:"'",end:"'",
illegal:"\\n",contains:[m]},O={scope:"string",begin:'"',end:'"',illegal:"\\n",
contains:[m]},C=(e,t,n={})=>{const r=i({scope:"comment",begin:e,end:t,
contains:[]},n);r.contains.push({scope:"doctag",
begin:"[ ]*(?=(TODO|FIXME|NOTE|BUG|OPTIMIZE|HACK|XXX):)",
end:/(TODO|FIXME|NOTE|BUG|OPTIMIZE|HACK|XXX):/,excludeBegin:!0,relevance:0})
;const s=u("I","a","is","so","us","to","at","if","in","it","on",/[A-Za-z]+['](d|ve|re|ll|t|s|n)/,/[A-Za-z]+[-][a-z]+/,/[A-Za-z][a-z]{2,}/)
;return r.contains.push({begin:g(/[ ]+/,"(",s,/[.]?[:]?([.][ ]|[ ])/,"){3}")}),r
},N=C("//","$"),B=C("/\\*","\\*/"),D=C("#","$");var S=Object.freeze({
__proto__:null,MATCH_NOTHING_RE:/\b\B/,IDENT_RE:f,UNDERSCORE_IDENT_RE:R,
NUMBER_RE:b,C_NUMBER_RE:M,BINARY_NUMBER_RE:E,
RE_STARTERS_RE:"!|!=|!==|%|%=|&|&&|&=|\\*|\\*=|\\+|\\+=|,|-|-=|/=|/|:|;|<<|<<=|<=|<|===|==|=|>>>=|>>=|>=|>>>|>>|>|\\?|\\[|\\{|\\(|\\^|\\^=|\\||\\|=|\\|\\||~",
SHEBANG:(e={})=>{const t=/^#![ ]*\//
;return e.binary&&(e.begin=g(t,/.*\b/,e.binary,/\b.*/)),i({scope:"meta",begin:t,
end:/$/,relevance:0,"on:begin":(e,t)=>{0!==e.index&&t.ignoreMatch()}},e)},
BACKSLASH_ESCAPE:m,APOS_STRING_MODE:T,QUOTE_STRING_MODE:O,PHRASAL_WORDS_MODE:{
begin:/\b(a|an|the|are|I'm|isn't|don't|doesn't|won't|but|just|should|pretty|simply|enough|gonna|going|wtf|so|such|will|you|your|they|like|more)\b/
},COMMENT:C,C_LINE_COMMENT_MODE:N,C_BLOCK_COMMENT_MODE:B,HASH_COMMENT_MODE:D,
NUMBER_MODE:{scope:"number",begin:b,relevance:0},C_NUMBER_MODE:{scope:"number",
begin:M,relevance:0},BINARY_NUMBER_MODE:{scope:"number",begin:E,relevance:0},
REGEXP_MODE:{begin:/(?=\/[^/\n]*\/)/,contains:[{scope:"regexp",begin:/\//,
end:/\/[gimuy]*/,illegal:/\n/,contains:[m,{begin:/\[/,end:/\]/,relevance:0,
contains:[m]}]}]},TITLE_MODE:{scope:"title",begin:f,relevance:0},
UNDERSCORE_TITLE_MODE:{scope:"title",begin:R,relevance:0},METHOD_GUARD:{
begin:"\\.\\s*[a-zA-Z_]\\w*",relevance:0},END_SAME_AS_BEGIN:e=>Object.assign(e,{
"on:begin":(e,t)=>{t.data._beginMatch=e[1]},"on:end":(e,t)=>{
t.data._beginMatch!==e[1]&&t.ignoreMatch()}})});function w(e,t){
"."===e.input[e.index-1]&&t.ignoreMatch()}function L(e,t){
void 0!==e.className&&(e.scope=e.className,delete e.className)}function y(e,t){
t&&e.beginKeywords&&(e.begin="\\b("+e.beginKeywords.split(" ").join("|")+")(?!\\.)(?=\\b|\\s)",
e.__beforeBegin=w,e.keywords=e.keywords||e.beginKeywords,delete e.beginKeywords,
void 0===e.relevance&&(e.relevance=0))}function x(e,t){
Array.isArray(e.illegal)&&(e.illegal=u(...e.illegal))}function I(e,t){
if(e.match){
if(e.begin||e.end)throw Error("begin & end are not supported with match")
;e.begin=e.match,delete e.match}}function F(e,t){
void 0===e.relevance&&(e.relevance=1)}const P=(e,t)=>{if(!e.beforeMatch)return
;if(e.starts)throw Error("beforeMatch cannot be used with starts")
;const n=Object.assign({},e);Object.keys(e).forEach((t=>{delete e[t]
})),e.keywords=n.keywords,
e.begin=g(n.beforeMatch,g("(?=",n.begin,")")),e.starts={relevance:0,
contains:[Object.assign(n,{endsParent:!0})]},e.relevance=0,delete n.beforeMatch
},v=["of","and","for","in","not","or","if","then","parent","list","value"]
;function U(e,t,n="keyword"){const r=Object.create(null)
;return"string"==typeof e?s(n,e.split(" ")):Array.isArray(e)?s(n,e):Object.keys(e).forEach((n=>{
Object.assign(r,U(e[n],t,n))})),r;function s(e,n){
t&&(n=n.map((e=>e.toLowerCase()))),n.forEach((t=>{const n=t.split("|")
;r[n[0]]=[e,k(n[0],n[1])]}))}}function k(e,t){
return t?Number(t):(e=>v.includes(e.toLowerCase()))(e)?0:1}const H={},j=e=>{
console.error(e)},G=(e,...t)=>{console.log("WARN: "+e,...t)},K=(e,t)=>{
H[`${e}/${t}`]||(console.log(`Deprecated as of ${e}. ${t}`),H[`${e}/${t}`]=!0)
},z=Error();function V(e,t,{key:n}){let r=0;const s=e[n],i={},a={}
;for(let e=1;e<=t.length;e++)a[e+r]=s[e],i[e+r]=!0,r+=h(t[e-1])
;e[n]=a,e[n]._emit=i,e[n]._multi=!0}function W(e){(e=>{
e.scope&&"object"==typeof e.scope&&null!==e.scope&&(e.beginScope=e.scope,
delete e.scope)})(e),"string"==typeof e.beginScope&&(e.beginScope={
_wrap:e.beginScope}),"string"==typeof e.endScope&&(e.endScope={_wrap:e.endScope
}),(e=>{if(Array.isArray(e.begin)){
if(e.skip||e.excludeBegin||e.returnBegin)throw j("skip, excludeBegin, returnBegin not compatible with beginScope: {}"),
z
;if("object"!=typeof e.beginScope||null===e.beginScope)throw j("beginScope must be object"),
z;V(e,e.begin,{key:"beginScope"}),e.begin=p(e.begin,{joinWith:""})}})(e),(e=>{
if(Array.isArray(e.end)){
if(e.skip||e.excludeEnd||e.returnEnd)throw j("skip, excludeEnd, returnEnd not compatible with endScope: {}"),
z
;if("object"!=typeof e.endScope||null===e.endScope)throw j("endScope must be object"),
z;V(e,e.end,{key:"endScope"}),e.end=p(e.end,{joinWith:""})}})(e)}function Y(e){
function t(t,n){return RegExp(_(t),"m"+(e.case_insensitive?"i":"")+(n?"g":""))}
class n{constructor(){
this.matchIndexes={},this.regexes=[],this.matchAt=1,this.position=0}
addRule(e,t){
t.position=this.position++,this.matchIndexes[this.matchAt]=t,this.regexes.push([t,e]),
this.matchAt+=h(e)+1}compile(){0===this.regexes.length&&(this.exec=()=>null)
;const e=this.regexes.map((e=>e[1]));this.matcherRe=t(p(e,{joinWith:"|"
}),!0),this.lastIndex=0}exec(e){this.matcherRe.lastIndex=this.lastIndex
;const t=this.matcherRe.exec(e);if(!t)return null
;const n=t.findIndex(((e,t)=>t>0&&void 0!==e)),r=this.matchIndexes[n]
;return t.splice(0,n),Object.assign(t,r)}}class r{constructor(){
this.rules=[],this.multiRegexes=[],
this.count=0,this.lastIndex=0,this.regexIndex=0}getMatcher(e){
if(this.multiRegexes[e])return this.multiRegexes[e];const t=new n
;return this.rules.slice(e).forEach((([e,n])=>t.addRule(e,n))),
t.compile(),this.multiRegexes[e]=t,t}resumingScanAtSamePosition(){
return 0!==this.regexIndex}considerAll(){this.regexIndex=0}addRule(e,t){
this.rules.push([e,t]),"begin"===t.type&&this.count++}exec(e){
const t=this.getMatcher(this.regexIndex);t.lastIndex=this.lastIndex
;let n=t.exec(e)
;if(this.resumingScanAtSamePosition())if(n&&n.index===this.lastIndex);else{
const t=this.getMatcher(0);t.lastIndex=this.lastIndex+1,n=t.exec(e)}
return n&&(this.regexIndex+=n.position+1,
this.regexIndex===this.count&&this.considerAll()),n}}
if(e.compilerExtensions||(e.compilerExtensions=[]),
e.contains&&e.contains.includes("self"))throw Error("ERR: contains `self` is not supported at the top-level of a language.  See documentation.")
;return e.classNameAliases=i(e.classNameAliases||{}),function n(s,a){const o=s
;if(s.isCompiled)return o
;[L,I,W,P].forEach((e=>e(s,a))),e.compilerExtensions.forEach((e=>e(s,a))),
s.__beforeBegin=null,[y,x,F].forEach((e=>e(s,a))),s.isCompiled=!0;let l=null
;return"object"==typeof s.keywords&&s.keywords.$pattern&&(s.keywords=Object.assign({},s.keywords),
l=s.keywords.$pattern,
delete s.keywords.$pattern),l=l||/\w+/,s.keywords&&(s.keywords=U(s.keywords,e.case_insensitive)),
o.keywordPatternRe=t(l,!0),
a&&(s.begin||(s.begin=/\B|\b/),o.beginRe=t(s.begin),s.end||s.endsWithParent||(s.end=/\B|\b/),
s.end&&(o.endRe=t(s.end)),
o.terminatorEnd=_(s.end)||"",s.endsWithParent&&a.terminatorEnd&&(o.terminatorEnd+=(s.end?"|":"")+a.terminatorEnd)),
s.illegal&&(o.illegalRe=t(s.illegal)),
s.contains||(s.contains=[]),s.contains=[].concat(...s.contains.map((e=>(e=>(e.variants&&!e.cachedVariants&&(e.cachedVariants=e.variants.map((t=>i(e,{
variants:null},t)))),e.cachedVariants?e.cachedVariants:$(e)?i(e,{
starts:e.starts?i(e.starts):null
}):Object.isFrozen(e)?i(e):e))("self"===e?s:e)))),s.contains.forEach((e=>{n(e,o)
})),s.starts&&n(s.starts,a),o.matcher=(e=>{const t=new r
;return e.contains.forEach((e=>t.addRule(e.begin,{rule:e,type:"begin"
}))),e.terminatorEnd&&t.addRule(e.terminatorEnd,{type:"end"
}),e.illegal&&t.addRule(e.illegal,{type:"illegal"}),t})(o),o}(e)}function $(e){
return!!e&&(e.endsWithParent||$(e.starts))}const X=s,Z=i,q=Symbol("nomatch")
;var J=(e=>{const t=Object.create(null),s=Object.create(null),i=[];let a=!0
;const o="Could not find the language '{}', did you forget to load/include a language module?",l={
disableAutodetect:!0,name:"Plain text",contains:[]};let _={
ignoreUnescapedHTML:!1,noHighlightRe:/^(no-?highlight)$/i,
languageDetectRe:/\blang(?:uage)?-([\w-]+)\b/i,classPrefix:"hljs-",
cssSelector:"pre code",languages:null,__emitter:c};function d(e){
return _.noHighlightRe.test(e)}function g(e,t,n){let r="",s=""
;"object"==typeof t?(r=e,
n=t.ignoreIllegals,s=t.language):(K("10.7.0","highlight(lang, code, ...args) has been deprecated."),
K("10.7.0","Please use highlight(code, options) instead.\nhttps://github.com/highlightjs/highlight.js/issues/2277"),
s=e,r=t),void 0===n&&(n=!0);const i={code:r,language:s};E("before:highlight",i)
;const a=i.result?i.result:u(i.language,i.code,n)
;return a.code=i.code,E("after:highlight",a),a}function u(e,n,s,i){
const l=Object.create(null);function c(){if(!C.keywords)return void B.addText(D)
;let e=0;C.keywordPatternRe.lastIndex=0;let t=C.keywordPatternRe.exec(D),n=""
;for(;t;){n+=D.substring(e,t.index)
;const s=m.case_insensitive?t[0].toLowerCase():t[0],i=(r=s,C.keywords[r]);if(i){
const[e,r]=i
;if(B.addText(n),n="",l[s]=(l[s]||0)+1,l[s]<=7&&(S+=r),e.startsWith("_"))n+=t[0];else{
const n=m.classNameAliases[e]||e;B.addKeyword(t[0],n)}}else n+=t[0]
;e=C.keywordPatternRe.lastIndex,t=C.keywordPatternRe.exec(D)}var r
;n+=D.substr(e),B.addText(n)}function d(){null!=C.subLanguage?(()=>{
if(""===D)return;let e=null;if("string"==typeof C.subLanguage){
if(!t[C.subLanguage])return void B.addText(D)
;e=u(C.subLanguage,D,!0,N[C.subLanguage]),N[C.subLanguage]=e._top
}else e=h(D,C.subLanguage.length?C.subLanguage:null)
;C.relevance>0&&(S+=e.relevance),B.addSublanguage(e._emitter,e.language)
})():c(),D=""}function g(e,t){let n=1;for(;void 0!==t[n];){if(!e._emit[n]){n++
;continue}const r=m.classNameAliases[e[n]]||e[n],s=t[n]
;r?B.addKeyword(s,r):(D=s,c(),D=""),n++}}function A(e,t){
return e.scope&&"string"==typeof e.scope&&B.openNode(m.classNameAliases[e.scope]||e.scope),
e.beginScope&&(e.beginScope._wrap?(B.addKeyword(D,m.classNameAliases[e.beginScope._wrap]||e.beginScope._wrap),
D=""):e.beginScope._multi&&(g(e.beginScope,t),D="")),C=Object.create(e,{parent:{
value:C}}),C}function p(e,t,n){let s=((e,t)=>{const n=e&&e.exec(t)
;return n&&0===n.index})(e.endRe,n);if(s){if(e["on:end"]){const n=new r(e)
;e["on:end"](t,n),n.isMatchIgnored&&(s=!1)}if(s){
for(;e.endsParent&&e.parent;)e=e.parent;return e}}
if(e.endsWithParent)return p(e.parent,t,n)}function f(e){
return 0===C.matcher.regexIndex?(D+=e[0],1):(y=!0,0)}function b(e){
const t=e[0],r=n.substr(e.index),s=p(C,e,r);if(!s)return q;const i=C
;C.endScope&&C.endScope._wrap?(d(),
B.addKeyword(t,C.endScope._wrap)):C.endScope&&C.endScope._multi?(d(),
g(C.endScope,e)):i.skip?D+=t:(i.returnEnd||i.excludeEnd||(D+=t),
d(),i.excludeEnd&&(D=t));do{
C.scope&&!C.isMultiClass&&B.closeNode(),C.skip||C.subLanguage||(S+=C.relevance),
C=C.parent}while(C!==s.parent)
;return s.starts&&A(s.starts,e),i.returnEnd?0:t.length}let M={};function E(t,i){
const o=i&&i[0];if(D+=t,null==o)return d(),0
;if("begin"===M.type&&"end"===i.type&&M.index===i.index&&""===o){
if(D+=n.slice(i.index,i.index+1),!a){const t=Error(`0 width match regex (${e})`)
;throw t.languageName=e,t.badRule=M.rule,t}return 1}
if(M=i,"begin"===i.type)return(e=>{
const t=e[0],n=e.rule,s=new r(n),i=[n.__beforeBegin,n["on:begin"]]
;for(const n of i)if(n&&(n(e,s),s.isMatchIgnored))return f(t)
;return n.skip?D+=t:(n.excludeBegin&&(D+=t),
d(),n.returnBegin||n.excludeBegin||(D=t)),A(n,e),n.returnBegin?0:t.length})(i)
;if("illegal"===i.type&&!s){
const e=Error('Illegal lexeme "'+o+'" for mode "'+(C.scope||"<unnamed>")+'"')
;throw e.mode=C,e}if("end"===i.type){const e=b(i);if(e!==q)return e}
if("illegal"===i.type&&""===o)return 1
;if(L>1e5&&L>3*i.index)throw Error("potential infinite loop, way more iterations than matches")
;return D+=o,o.length}const m=R(e)
;if(!m)throw j(o.replace("{}",e)),Error('Unknown language: "'+e+'"')
;const T=Y(m);let O="",C=i||T;const N={},B=new _.__emitter(_);(()=>{const e=[]
;for(let t=C;t!==m;t=t.parent)t.scope&&e.unshift(t.scope)
;e.forEach((e=>B.openNode(e)))})();let D="",S=0,w=0,L=0,y=!1;try{
for(C.matcher.considerAll();;){
L++,y?y=!1:C.matcher.considerAll(),C.matcher.lastIndex=w
;const e=C.matcher.exec(n);if(!e)break;const t=E(n.substring(w,e.index),e)
;w=e.index+t}return E(n.substr(w)),B.closeAllNodes(),B.finalize(),O=B.toHTML(),{
language:e,value:O,relevance:S,illegal:!1,_emitter:B,_top:C}}catch(t){
if(t.message&&t.message.includes("Illegal"))return{language:e,value:X(n),
illegal:!0,relevance:0,_illegalBy:{message:t.message,index:w,
context:n.slice(w-100,w+100),mode:t.mode,resultSoFar:O},_emitter:B};if(a)return{
language:e,value:X(n),illegal:!1,relevance:0,errorRaised:t,_emitter:B,_top:C}
;throw t}}function h(e,n){n=n||_.languages||Object.keys(t);const r=(e=>{
const t={value:X(e),illegal:!1,relevance:0,_top:l,_emitter:new _.__emitter(_)}
;return t._emitter.addText(e),t})(e),s=n.filter(R).filter(M).map((t=>u(t,e,!1)))
;s.unshift(r);const i=s.sort(((e,t)=>{
if(e.relevance!==t.relevance)return t.relevance-e.relevance
;if(e.language&&t.language){if(R(e.language).supersetOf===t.language)return 1
;if(R(t.language).supersetOf===e.language)return-1}return 0})),[a,o]=i,c=a
;return c.secondBest=o,c}function A(e){let t=null;const n=(e=>{
let t=e.className+" ";t+=e.parentNode?e.parentNode.className:""
;const n=_.languageDetectRe.exec(t);if(n){const t=R(n[1])
;return t||(G(o.replace("{}",n[1])),
G("Falling back to no-highlight mode for this block.",e)),t?n[1]:"no-highlight"}
return t.split(/\s+/).find((e=>d(e)||R(e)))})(e);if(d(n))return
;E("before:highlightElement",{el:e,language:n
}),!_.ignoreUnescapedHTML&&e.children.length>0&&(console.warn("One of your code blocks includes unescaped HTML. This is a potentially serious security risk."),
console.warn("https://github.com/highlightjs/highlight.js/issues/2886"),
console.warn(e)),t=e;const r=t.textContent,i=n?g(r,{language:n,ignoreIllegals:!0
}):h(r);e.innerHTML=i.value,((e,t,n)=>{const r=t&&s[t]||n
;e.classList.add("hljs"),e.classList.add("language-"+r)
})(e,n,i.language),e.result={language:i.language,re:i.relevance,
relevance:i.relevance},i.secondBest&&(e.secondBest={
language:i.secondBest.language,relevance:i.secondBest.relevance
}),E("after:highlightElement",{el:e,result:i,text:r})}let p=!1;function f(){
"loading"!==document.readyState?document.querySelectorAll(_.cssSelector).forEach(A):p=!0
}function R(e){return e=(e||"").toLowerCase(),t[e]||t[s[e]]}
function b(e,{languageName:t}){"string"==typeof e&&(e=[e]),e.forEach((e=>{
s[e.toLowerCase()]=t}))}function M(e){const t=R(e)
;return t&&!t.disableAutodetect}function E(e,t){const n=e;i.forEach((e=>{
e[n]&&e[n](t)}))}
"undefined"!=typeof window&&window.addEventListener&&window.addEventListener("DOMContentLoaded",(()=>{
p&&f()}),!1),Object.assign(e,{highlight:g,highlightAuto:h,highlightAll:f,
highlightElement:A,
highlightBlock:e=>(K("10.7.0","highlightBlock will be removed entirely in v12.0"),
K("10.7.0","Please use highlightElement now."),A(e)),configure:e=>{_=Z(_,e)},
initHighlighting:()=>{
f(),K("10.6.0","initHighlighting() deprecated.  Use highlightAll() now.")},
initHighlightingOnLoad:()=>{
f(),K("10.6.0","initHighlightingOnLoad() deprecated.  Use highlightAll() now.")
},registerLanguage:(n,r)=>{let s=null;try{s=r(e)}catch(e){
if(j("Language definition for '{}' could not be registered.".replace("{}",n)),
!a)throw e;j(e),s=l}
s.name||(s.name=n),t[n]=s,s.rawDefinition=r.bind(null,e),s.aliases&&b(s.aliases,{
languageName:n})},unregisterLanguage:e=>{delete t[e]
;for(const t of Object.keys(s))s[t]===e&&delete s[t]},
listLanguages:()=>Object.keys(t),getLanguage:R,registerAliases:b,
autoDetection:M,inherit:Z,addPlugin:e=>{(e=>{
e["before:highlightBlock"]&&!e["before:highlightElement"]&&(e["before:highlightElement"]=t=>{
e["before:highlightBlock"](Object.assign({block:t.el},t))
}),e["after:highlightBlock"]&&!e["after:highlightElement"]&&(e["after:highlightElement"]=t=>{
e["after:highlightBlock"](Object.assign({block:t.el},t))})})(e),i.push(e)}
}),e.debugMode=()=>{a=!1},e.safeMode=()=>{a=!0},e.versionString="11.1.0"
;for(const e in S)"object"==typeof S[e]&&n(S[e]);return Object.assign(e,S),e
})({}),Q=Object.freeze({__proto__:null,grmr_rgbasm:e=>{const t={
className:"subst",begin:/\{/,end:/\}/,contains:[{className:"variable",
variants:[{match:/[a-z_][a-z0-9_#@]*/},{
match:/(?:[a-z_][a-z0-9_#@]*)?\.[a-z0-9_#@]*/}]}],illegal:/[^a-z0-9_@#.\\}+ -]/}
;let n=e.COMMENT(/\/\*/,/\*\//);return n.relevance=0,{name:"RGBASM",
case_insensitive:!0,aliases:["rgbds","gbasm","gbz80"],keywords:{
$pattern:/[a-z_][a-z0-9_#@]*/,
keyword:"adc add and bit call ccf cpl cp daa dec di ei halt inc jp jr ld ldi ldd ldio ldh nop or pop push res reti ret rlca rlc rla rl rrc rrca rra rr rst sbc scf set sla sra srl stop sub swap xor def bank align round ceil floor div mul sin cos tan asin acos atan atan2 pow log high low isconst sizeof startof strcmp strin strsub strlen strcat strupr strlwr strrin strrpl strfmt include print println printt printi printv printf export ds|0 db|0 dw|0 dl|0 section purge rsreset rsset incbin|10 charmap|10 newcharmap|10 setcharmap|10 fail warn fatal assert static_assert shift opt break macro endm rept for endr load endl pushc popc union nextu endu pushs pops pusho popo if|0 else|0 elif|0 endc|0 rb rw equ equs redef",
literal:["_NARG","_RS","__LINE__","__FILE__","__DATE__","__TIME__","__ISO_8601_LOCAL__","__ISO_8601_UTC__","__UTC_YEAR__","__UTC_MONTH__","__UTC_DAY__","__UTC_HOUR__","__UTC_MINUTE__","__UTC_SECOND__","__RGBDS_MAJOR__","__RGBDS_MINOR__","__RGBDS_PATCH__","__RGBDS_VERSION__"],
_hardware_inc:"_VRAM _VRAM8000 _VRAM8800 _VRAM9000 _SCRN0 _SCRN1 _SRAM _RAM _RAMBANK _OAMRAM _IO _AUD3WAVERAM _HRAM rRAMG rROMB0 rROMB1 rRAMB rP1 P1F_5 P1F_4 P1F_3 P1F_2 P1F_1 P1F_0 P1F_GET_DPAD P1F_GET_BTN P1F_GET_NONE rSB rSC rDIV rTIMA rTMA rTAC TACF_START TACF_STOP TACF_4KHZ TACF_16KHZ TACF_65KHZ TACF_262KHZ rIF rNR10 rAUD1SWEEP AUD1SWEEP_UP AUD1SWEEP_DOWN rNR11 rAUD1LEN rNR12 rAUD1ENV rNR13 rAUD1LOW rNR14 rAUD1HIGH rNR21 rAUD2LEN rNR22 rAUD2ENV rNR23 rAUD2LOW rNR24 rAUD2HIGH rNR30 rAUD3ENA rNR31 rAUD3LEN rNR32 rAUD3LEVEL rNR33 rAUD3LOW rNR34 rAUD3HIGH rNR41 rAUD4LEN rNR42 rAUD4ENV rNR43 rAUD4POLY rNR44 rAUD4GO rNR50 rAUDVOL AUDVOL_VIN_LEFT AUDVOL_VIN_RIGHT rNR51 rAUDTERM AUDTERM_4_LEFT AUDTERM_3_LEFT AUDTERM_2_LEFT AUDTERM_1_LEFT AUDTERM_4_RIGHT AUDTERM_3_RIGHT AUDTERM_2_RIGHT AUDTERM_1_RIGHT rNR52 rAUDENA AUDENA_ON AUDENA_OFF rLCDC LCDCF_OFF LCDCF_ON LCDCF_WIN9800 LCDCF_WIN9C00 LCDCF_WINOFF LCDCF_WINON LCDCF_BG8800 LCDCF_BG8000 LCDCF_BG9800 LCDCF_BG9C00 LCDCF_OBJ8 LCDCF_OBJ16 LCDCF_OBJOFF LCDCF_OBJON LCDCF_BGOFF LCDCF_BGON rSTAT STATF_LYC STATF_MODE10 STATF_MODE01 STATF_MODE00 STATF_LYCF STATF_HBL STATF_VBL STATF_OAM STATF_LCD STATF_BUSY rSCY rSCX rLY rLYC rDMA rBGP rOBP0 rOBP1 rWY rWX rKEY1 rSPD KEY1F_DBLSPEED KEY1F_PREPARE rVBK rHDMA1 rHDMA2 rHDMA3 rHDMA4 rHDMA5 HDMA5F_MODE_GP HDMA5F_MODE_HBL HDMA5F_BUSY rRP RPF_ENREAD RPF_DATAIN RPF_WRITE_HI RPF_WRITE_LO rBCPS BCPSF_AUTOINC rBCPD rOCPS OCPSF_AUTOINC rOCPD rSVBK rSMBK rPCM12 rPCM34 rIE IEF_HILO IEF_SERIAL IEF_TIMER IEF_STAT IEF_VBLANK AUDLEN_DUTY_12_5 AUDLEN_DUTY_25 AUDLEN_DUTY_50 AUDLEN_DUTY_75 AUDENV_UP AUDENV_DOWN AUDHIGH_RESTART AUDHIGH_LENGTH_ON AUDHIGH_LENGTH_OFF BOOTUP_A_DMG BOOTUP_A_CGB BOOTUP_A_MGB BOOTUP_B_CGB BOOTUP_B_AGB CART_COMPATIBLE_DMG CART_COMPATIBLE_DMG_GBC CART_COMPATIBLE_GBC CART_INDICATOR_GB CART_INDICATOR_SGB CART_ROM CART_ROM_MBC1 CART_ROM_MBC1_RAM CART_ROM_MBC1_RAM_BAT CART_ROM_MBC2 CART_ROM_MBC2_BAT CART_ROM_RAM CART_ROM_RAM_BAT CART_ROM_MMM01 CART_ROM_MMM01_RAM CART_ROM_MMM01_RAM_BAT CART_ROM_MBC3_BAT_RTC CART_ROM_MBC3_RAM_BAT_RTC CART_ROM_MBC3 CART_ROM_MBC3_RAM CART_ROM_MBC3_RAM_BAT CART_ROM_MBC5 CART_ROM_MBC5_BAT CART_ROM_MBC5_RAM_BAT CART_ROM_MBC5_RUMBLE CART_ROM_MBC5_RAM_RUMBLE CART_ROM_MBC5_RAM_BAT_RUMBLE CART_ROM_MBC7_RAM_BAT_GYRO CART_ROM_POCKET_CAMERA CART_ROM_BANDAI_TAMA5 CART_ROM_HUDSON_HUC3 CART_ROM_HUDSON_HUC1 CART_ROM_32KB CART_ROM_64KB CART_ROM_128KB CART_ROM_256KB CART_ROM_512KB CART_ROM_1024KB CART_ROM_2048KB CART_ROM_4096KB CART_ROM_8192KB CART_ROM_1152KB CART_ROM_1280KB CART_ROM_1536KB CART_SRAM_NONE CART_SRAM_2KB CART_SRAM_8KB CART_SRAM_32KB CART_SRAM_128KB CART_SRAM_ENABLE CART_SRAM_DISABLE CART_DEST_JAPANESE CART_DEST_NON_JAPANESE PADF_DOWN PADF_UP PADF_LEFT PADF_RIGHT PADF_START PADF_SELECT PADF_B PADF_A PADB_DOWN PADB_UP PADB_LEFT PADB_RIGHT PADB_START PADB_SELECT PADB_B PADB_A SCRN_X SCRN_Y SCRN_X_B SCRN_Y_B SCRN_VX SCRN_VY SCRN_VX_B SCRN_VY_B OAMA_Y OAMA_X OAMA_TILEID OAMA_FLAGS sizeof_OAM_ATTRS OAM_COUNT OAMF_PRI OAMF_YFLIP OAMF_XFLIP OAMF_PAL0 OAMF_PAL1 OAMF_BANK0 OAMF_BANK1 OAMF_PALMASK OAMB_PRI OAMB_YFLIP OAMB_XFLIP OAMB_PAL1 OAMB_BANK1 IEF_LCDC"
},contains:[e.COMMENT(/;/,/$/),n,t,{className:"number",variants:[{
match:/\$[0-9-af]+/},{match:/\b[0-9]+(\.[0-9]+)?/,relevance:0},{match:/&[0-7]+/
},{match:/%[01]+/},{match:/`[0-3]+/}]},{className:"operator",
match:/\*\*|~|\+|-|\*|\/|%|<<|>>|&|\||\^|!=|==|<=|>=|<|>|&&|\|\||!/,relevance:0
},{className:"punctuation",match:/[,[\]:]/,relevance:0},{className:"string",
begin:/"/,end:/"/,contains:[t],relevance:0},{className:"symbol",variants:[{
match:/^[ \t]*[a-z_][a-z0-9_#@]*(?=:)/},{
match:/^[ \t]*(?:[a-z_][a-z0-9_#@]*)?\.[a-z0-9_#@]+(?![a-z0-9_#@])/}],
relevance:0},{className:"type",
match:/\b(?:wram0|vram|romx|rom0|hram|wramx|sram|oam)(?![a-z0-9_#@])/},{
className:"variable",
match:/\b(?:af|bc|de|hl|hli|hld|a|b|c|d|e|h|l|nz|z|nc)(?![a-z0-9_#@])/,
relevance:0}],
illegal:[/\.[a-z0-9_#@]*\./,/\.[0-9]*\./,/^[ \t]*[^ \t\r\na-z_:;]/]}},
grmr_c:e=>{const t=e.COMMENT("//","$",{contains:[{begin:/\\\n/}]
}),n="[a-zA-Z_]\\w*::",r="(decltype\\(auto\\)|"+d(n)+"[a-zA-Z_]\\w*"+d("<[^<>]+>")+")",s={
className:"type",variants:[{begin:"\\b[a-z\\d_]*_t\\b"},{
match:/\batomic_[a-z]{3,6}\b/}]},i={className:"string",variants:[{
begin:'(u8?|U|L)?"',end:'"',illegal:"\\n",contains:[e.BACKSLASH_ESCAPE]},{
begin:"(u8?|U|L)?'(\\\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4,8}|[0-7]{3}|\\S)|.)",
end:"'",illegal:"."},e.END_SAME_AS_BEGIN({
begin:/(?:u8?|U|L)?R"([^()\\ ]{0,16})\(/,end:/\)([^()\\ ]{0,16})"/})]},a={
className:"number",variants:[{begin:"\\b(0b[01']+)"},{
begin:"(-?)\\b([\\d']+(\\.[\\d']*)?|\\.[\\d']+)((ll|LL|l|L)(u|U)?|(u|U)(ll|LL|l|L)?|f|F|b|B)"
},{
begin:"(-?)(\\b0[xX][a-fA-F0-9']+|(\\b[\\d']+(\\.[\\d']*)?|\\.[\\d']+)([eE][-+]?[\\d']+)?)"
}],relevance:0},o={className:"meta",begin:/#\s*[a-z]+\b/,end:/$/,keywords:{
keyword:"if else elif endif define undef warning error line pragma _Pragma ifdef ifndef include"
},contains:[{begin:/\\\n/,relevance:0},e.inherit(i,{className:"string"}),{
className:"string",begin:/<.*?>/},t,e.C_BLOCK_COMMENT_MODE]},l={
className:"title",begin:d(n)+e.IDENT_RE,relevance:0
},c=d(n)+e.IDENT_RE+"\\s*\\(",_={
keyword:["asm","auto","break","case","const","continue","default","do","else","enum","extern","for","fortran","goto","if","inline","register","restrict","return","sizeof","static","struct","switch","typedef","union","volatile","while","_Alignas","_Alignof","_Atomic","_Generic","_Noreturn","_Static_assert","_Thread_local","alignas","alignof","noreturn","static_assert","thread_local","_Pragma"],
type:["float","double","signed","unsigned","int","short","long","char","void","_Bool","_Complex","_Imaginary","_Decimal32","_Decimal64","_Decimal128","complex","bool","imaginary"],
literal:"true false NULL",
built_in:"std string wstring cin cout cerr clog stdin stdout stderr stringstream istringstream ostringstream auto_ptr deque list queue stack vector map set pair bitset multiset multimap unordered_set unordered_map unordered_multiset unordered_multimap priority_queue make_pair array shared_ptr abort terminate abs acos asin atan2 atan calloc ceil cosh cos exit exp fabs floor fmod fprintf fputs free frexp fscanf future isalnum isalpha iscntrl isdigit isgraph islower isprint ispunct isspace isupper isxdigit tolower toupper labs ldexp log10 log malloc realloc memchr memcmp memcpy memset modf pow printf putchar puts scanf sinh sin snprintf sprintf sqrt sscanf strcat strchr strcmp strcpy strcspn strlen strncat strncmp strncpy strpbrk strrchr strspn strstr tanh tan vfprintf vprintf vsprintf endl initializer_list unique_ptr"
},g=[o,s,t,e.C_BLOCK_COMMENT_MODE,a,i],u={variants:[{begin:/=/,end:/;/},{
begin:/\(/,end:/\)/},{beginKeywords:"new throw return else",end:/;/}],
keywords:_,contains:g.concat([{begin:/\(/,end:/\)/,keywords:_,
contains:g.concat(["self"]),relevance:0}]),relevance:0},h={
begin:"("+r+"[\\*&\\s]+)+"+c,returnBegin:!0,end:/[{;=]/,excludeEnd:!0,
keywords:_,illegal:/[^\w\s\*&:<>.]/,contains:[{begin:"decltype\\(auto\\)",
keywords:_,relevance:0},{begin:c,returnBegin:!0,contains:[e.inherit(l,{
className:"title.function"})],relevance:0},{relevance:0,match:/,/},{
className:"params",begin:/\(/,end:/\)/,keywords:_,relevance:0,
contains:[t,e.C_BLOCK_COMMENT_MODE,i,a,s,{begin:/\(/,end:/\)/,keywords:_,
relevance:0,contains:["self",t,e.C_BLOCK_COMMENT_MODE,i,a,s]}]
},s,t,e.C_BLOCK_COMMENT_MODE,o]};return{name:"C",aliases:["h"],keywords:_,
disableAutodetect:!0,illegal:"</",contains:[].concat(u,h,g,[o,{
begin:e.IDENT_RE+"::",keywords:_},{className:"class",
beginKeywords:"enum class struct union",end:/[{;:<>=]/,contains:[{
beginKeywords:"final class struct"},e.TITLE_MODE]}]),exports:{preprocessor:o,
strings:i,keywords:_}}}});const ee=J;for(const e of Object.keys(Q)){
const t=e.replace("grmr_","").replace("_","-");ee.registerLanguage(t,Q[e])}
return ee}()
;"object"==typeof exports&&"undefined"!=typeof module&&(module.exports=hljs);