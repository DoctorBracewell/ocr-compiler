const F=function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))o(r);new MutationObserver(r=>{for(const i of r)if(i.type==="childList")for(const c of i.addedNodes)c.tagName==="LINK"&&c.rel==="modulepreload"&&o(c)}).observe(document,{childList:!0,subtree:!0});function n(r){const i={};return r.integrity&&(i.integrity=r.integrity),r.referrerpolicy&&(i.referrerPolicy=r.referrerpolicy),r.crossorigin==="use-credentials"?i.credentials="include":r.crossorigin==="anonymous"?i.credentials="omit":i.credentials="same-origin",i}function o(r){if(r.ep)return;r.ep=!0;const i=n(r);fetch(r.href,i)}};F();function _(){}function M(e){return e()}function N(){return Object.create(null)}function b(e){e.forEach(M)}function D(e){return typeof e=="function"}function H(e,t){return e!=e?t==t:e!==t||e&&typeof e=="object"||typeof e=="function"}function G(e){return Object.keys(e).length===0}function d(e,t){e.appendChild(t)}function P(e,t,n){e.insertBefore(t,n||null)}function L(e){e.parentNode.removeChild(e)}function p(e){return document.createElement(e)}function w(e){return document.createTextNode(e)}function v(){return w(" ")}function J(e,t,n,o){return e.addEventListener(t,n,o),()=>e.removeEventListener(t,n,o)}function Q(e,t,n){n==null?e.removeAttribute(t):e.getAttribute(t)!==n&&e.setAttribute(t,n)}function R(e){return Array.from(e.childNodes)}function U(e,t){t=""+t,e.wholeText!==t&&(e.data=t)}let C;function m(e){C=e}const h=[],S=[],y=[],T=[],W=Promise.resolve();let k=!1;function X(){k||(k=!0,W.then(B))}function E(e){y.push(e)}const x=new Set;let g=0;function B(){const e=C;do{for(;g<h.length;){const t=h[g];g++,m(t),Y(t.$$)}for(m(null),h.length=0,g=0;S.length;)S.pop()();for(let t=0;t<y.length;t+=1){const n=y[t];x.has(n)||(x.add(n),n())}y.length=0}while(h.length);for(;T.length;)T.pop()();k=!1,x.clear(),m(e)}function Y(e){if(e.fragment!==null){e.update(),b(e.before_update);const t=e.dirty;e.dirty=[-1],e.fragment&&e.fragment.p(e.ctx,t),e.after_update.forEach(E)}}const $=new Set;let Z;function I(e,t){e&&e.i&&($.delete(e),e.i(t))}function ee(e,t,n,o){if(e&&e.o){if($.has(e))return;$.add(e),Z.c.push(()=>{$.delete(e),o&&(n&&e.d(1),o())}),e.o(t)}}function te(e){e&&e.c()}function K(e,t,n,o){const{fragment:r,on_mount:i,on_destroy:c,after_update:l}=e.$$;r&&r.m(t,n),o||E(()=>{const u=i.map(M).filter(D);c?c.push(...u):b(u),e.$$.on_mount=[]}),l.forEach(E)}function V(e,t){const n=e.$$;n.fragment!==null&&(b(n.on_destroy),n.fragment&&n.fragment.d(t),n.on_destroy=n.fragment=null,n.ctx=[])}function ne(e,t){e.$$.dirty[0]===-1&&(h.push(e),X(),e.$$.dirty.fill(0)),e.$$.dirty[t/31|0]|=1<<t%31}function q(e,t,n,o,r,i,c,l=[-1]){const u=C;m(e);const s=e.$$={fragment:null,ctx:null,props:i,update:_,not_equal:r,bound:N(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(t.context||(u?u.$$.context:[])),callbacks:N(),dirty:l,skip_bound:!1,root:t.target||u.$$.root};c&&c(s.root);let f=!1;if(s.ctx=n?n(e,t.props||{},(a,O,...j)=>{const A=j.length?j[0]:O;return s.ctx&&r(s.ctx[a],s.ctx[a]=A)&&(!s.skip_bound&&s.bound[a]&&s.bound[a](A),f&&ne(e,a)),O}):[],s.update(),f=!0,b(s.before_update),s.fragment=o?o(s.ctx):!1,t.target){if(t.hydrate){const a=R(t.target);s.fragment&&s.fragment.l(a),a.forEach(L)}else s.fragment&&s.fragment.c();t.intro&&I(e.$$.fragment),K(e,t.target,t.anchor,t.customElement),B()}m(u)}class z{$destroy(){V(this,1),this.$destroy=_}$on(t,n){const o=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return o.push(n),()=>{const r=o.indexOf(n);r!==-1&&o.splice(r,1)}}$set(t){this.$$set&&!G(t)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}function re(e){let t,n,o,r,i;return{c(){t=p("button"),n=w("Clicks: "),o=w(e[0]),Q(t,"class","svelte-5fj0zs")},m(c,l){P(c,t,l),d(t,n),d(t,o),r||(i=J(t,"click",e[1]),r=!0)},p(c,[l]){l&1&&U(o,c[0])},i:_,o:_,d(c){c&&L(t),r=!1,i()}}}function oe(e,t,n){let o=0;return[o,()=>{n(0,o+=1)}]}class ie extends z{constructor(t){super(),q(this,t,oe,re,H,{})}}function se(e){let t,n,o,r,i,c,l,u,s;return r=new ie({}),{c(){t=p("main"),n=p("h1"),n.textContent="Hello Typescript!",o=v(),te(r.$$.fragment),i=v(),c=p("p"),c.innerHTML='Visit <a href="https://svelte.dev">svelte.dev</a> to learn how to build Svelte apps.',l=v(),u=p("p"),u.innerHTML=`Check out <a href="https://github.com/sveltejs/kit#readme">SvelteKit</a> for the officially supported
		framework, also powered by Vite!`},m(f,a){P(f,t,a),d(t,n),d(t,o),K(r,t,null),d(t,i),d(t,c),d(t,l),d(t,u),s=!0},p:_,i(f){s||(I(r.$$.fragment,f),s=!0)},o(f){ee(r.$$.fragment,f),s=!1},d(f){f&&L(t),V(r)}}}class ce extends z{constructor(t){super(),q(this,t,null,se,H,{})}}new ce({target:document.getElementById("app")});
