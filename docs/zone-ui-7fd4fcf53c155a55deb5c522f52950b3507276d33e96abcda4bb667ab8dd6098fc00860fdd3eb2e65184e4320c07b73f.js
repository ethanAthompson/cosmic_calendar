let Q=1,a5=576,a4=487,P=null,R=`undefined`,X=`boolean`,Y=`string`,U=0,_=`Object`,S=`utf-8`,W=`number`,$=4,V=`function`,a3=11,N=Array,Z=Array.isArray,a2=Date,T=Error,a1=Object,a0=Reflect,O=undefined;var E=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h418eebf2303e9308(b,c,g(d))});var u=(a=>{const b=typeof a;if(b==W||b==X||a==P){return `${a}`};if(b==Y){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==Y&&b.length>U){return `Function(${b})`}else{return `Function`}};if(Z(a)){const b=a.length;let c=`[`;if(b>U){c+=u(a[U])};for(let d=Q;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Q){d=c[Q]}else{return toString.call(a)};if(d==_){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return _}};if(a instanceof T){return `${a.name}: ${a.message}\n${a.stack}`};return d});var J=((a,b)=>{});var x=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h481097dac51739bc(b,c)});function F(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(g(b))}}var p=(a=>a===O||a===P);var c=(a=>b[a]);var z=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h196d61d35f86e0f1(b,c,g(d))});var t=(()=>{if(s===P||s.byteLength===U){s=new Float64Array(a.memory.buffer)};return s});var I=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return g(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==Q){b.a=U;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return g(c)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===Y?e:O;var g=p(f)?U:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/$+ Q]=h;r()[b/$+ U]=g});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Y;return b});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===W?d:O;t()[a/8+ Q]=p(e)?U:e;r()[a/$+ U]=!p(e)});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===X?(b?Q:U):2;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===P;return b});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_d609202d16c38224=(a=>{const b=c(a).document;return p(b)?U:g(b)});b.wbg.__wbg_location_176c34e89c2c9d80=(a=>{const b=c(a).location;return g(b)});b.wbg.__wbg_history_80998b7456bf367e=function(){return F((a=>{const b=c(a).history;return g(b)}),arguments)};b.wbg.__wbg_localStorage_8c507fd281456944=function(){return F((a=>{const b=c(a).localStorage;return p(b)?U:g(b)}),arguments)};b.wbg.__wbg_sessionStorage_adb12b0c8ea06c48=function(){return F((a=>{const b=c(a).sessionStorage;return p(b)?U:g(b)}),arguments)};b.wbg.__wbg_matchMedia_7fbd33cb577fe4ad=function(){return F(((a,b,d)=>{var e=G(b,d);const f=c(a).matchMedia(e);return p(f)?U:g(f)}),arguments)};b.wbg.__wbg_scrollTo_eb21c4452d7b3cd6=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbg_requestAnimationFrame_74309aadebde12fa=function(){return F(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_clearTimeout_0f534a4b1fb4773d=((a,b)=>{c(a).clearTimeout(b)});b.wbg.__wbg_fetch_6c415b3a07763878=((a,b)=>{const d=c(a).fetch(c(b));return g(d)});b.wbg.__wbg_setInterval_edbd739de0ac5430=function(){return F(((a,b,d)=>{const e=c(a).setInterval(c(b),d);return e}),arguments)};b.wbg.__wbg_setTimeout_06458eba2b40711c=function(){return F(((a,b,d)=>{const e=c(a).setTimeout(c(b),d);return e}),arguments)};b.wbg.__wbg_body_64abc9aba1891e91=(a=>{const b=c(a).body;return p(b)?U:g(b)});b.wbg.__wbg_createComment_529b047c02bbe600=((a,b,d)=>{var e=G(b,d);const f=c(a).createComment(e);return g(f)});b.wbg.__wbg_createDocumentFragment_1c6d6aeeb8a8eb2e=(a=>{const b=c(a).createDocumentFragment();return g(b)});b.wbg.__wbg_createElement_fdd5c113cb84539e=function(){return F(((a,b,d)=>{var e=G(b,d);const f=c(a).createElement(e);return g(f)}),arguments)};b.wbg.__wbg_createElementNS_524b05a6070757b6=function(){return F(((a,b,d,e,f)=>{var h=G(b,d);var i=G(e,f);const j=c(a).createElementNS(h,i);return g(j)}),arguments)};b.wbg.__wbg_createTextNode_7ff0c034b2855f66=((a,b,d)=>{var e=G(b,d);const f=c(a).createTextNode(e);return g(f)});b.wbg.__wbg_getElementById_65b9547a428b5eb4=((a,b,d)=>{var e=G(b,d);const f=c(a).getElementById(e);return p(f)?U:g(f)});b.wbg.__wbg_id_ba8ed2468700af37=((b,d)=>{const e=c(d).id;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_setclassName_4b720df7b12cb674=((a,b,d)=>{var e=G(b,d);c(a).className=e});b.wbg.__wbg_classList_82893a9100db6428=(a=>{const b=c(a).classList;return g(b)});b.wbg.__wbg_setinnerHTML_ce0d6527ce4086f2=((a,b,d)=>{var e=G(b,d);c(a).innerHTML=e});b.wbg.__wbg_getAttribute_bff489553dd803cc=((b,d,e,f)=>{var g=G(e,f);const h=c(d).getAttribute(g);var i=p(h)?U:o(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;r()[b/$+ Q]=j;r()[b/$+ U]=i});b.wbg.__wbg_hasAttribute_bfb8f7140cf587f1=((a,b,d)=>{var e=G(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_querySelectorAll_3e2bd695ce88c618=function(){return F(((a,b,d)=>{var e=G(b,d);const f=c(a).querySelectorAll(e);return g(f)}),arguments)};b.wbg.__wbg_removeAttribute_2e200daefb9f3ed4=function(){return F(((a,b,d)=>{var e=G(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_scrollIntoView_3de22d537ed95550=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_setAttribute_e7b72a5e7cfcb5a3=function(){return F(((a,b,d,e,f)=>{var g=G(b,d);var h=G(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_74a825a7b3d13d06=function(){return F(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_0d26d36fd4f25c4e=(a=>{c(a).remove()});b.wbg.__wbg_append_df44ca631c3c1657=function(){return F(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_instanceof_HtmlElement_55a0f0f0f0f0118e=(a=>{let b;try{b=c(a) instanceof HTMLElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithform_f67de494d7d454b2=function(){return F((a=>{const b=new FormData(c(a));return g(b)}),arguments)};b.wbg.__wbg_pushState_e159043fce8f87bc=function(){return F(((a,b,d,e,f,g)=>{var h=G(d,e);var i=G(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_replaceState_b51dd62c7235b1ac=function(){return F(((a,b,d,e,f,g)=>{var h=G(d,e);var i=G(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_sethref_90b000c8b01f96b1=function(){return F(((a,b,d)=>{var e=G(b,d);c(a).href=e}),arguments)};b.wbg.__wbg_origin_595edc88be6e66b8=function(){return F(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_protocol_51a4e630fff75abb=function(){return F(((b,d)=>{const e=c(d).protocol;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_hostname_6a864c6261102cc7=function(){return F(((b,d)=>{const e=c(d).hostname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_port_715b7b0dc92c5688=function(){return F(((b,d)=>{const e=c(d).port;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_pathname_1ab7e82aaa4511ff=function(){return F(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_search_9f7ca8896c2d0804=function(){return F(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_hash_be2940ca236b5efc=function(){return F(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f}),arguments)};b.wbg.__wbg_origin_aab6d2be79bcec84=((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_pathname_aeafa820be91c325=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_search_f6e95882a48d3f69=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_setsearch_4f7d084e0d811add=((a,b,d)=>{var e=G(b,d);c(a).search=e});b.wbg.__wbg_searchParams_00f98167a3c8c4da=(a=>{const b=c(a).searchParams;return g(b)});b.wbg.__wbg_hash_0087751acddc8f2a=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_new_9e08fd37c1c53142=function(){return F(((a,b)=>{var c=G(a,b);const d=new URL(c);return g(d)}),arguments)};b.wbg.__wbg_newwithbase_f4989aa5bbd5cc29=function(){return F(((a,b,c,d)=>{var e=G(a,b);var f=G(c,d);const h=new URL(e,f);return g(h)}),arguments)};b.wbg.__wbg_instanceof_HtmlButtonElement_edc54e80ec7dfee1=(a=>{let b;try{b=c(a) instanceof HTMLButtonElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_detail_0587ac8d20a4cdd4=(a=>{const b=c(a).detail;return g(b)});b.wbg.__wbg_newwitheventinitdict_4444ad4e8ce3d9dd=function(){return F(((a,b,d)=>{var e=G(a,b);const f=new CustomEvent(e,c(d));return g(f)}),arguments)};b.wbg.__wbg_instanceof_HtmlInputElement_e7869aaef9cbb0e6=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_value_e024243a9dae20bc=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_submitter_c3372678ca5888e0=(a=>{const b=c(a).submitter;return p(b)?U:g(b)});b.wbg.__wbg_append_962e199b73af5069=function(){return F(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_parentNode_92a7017b3a4fad43=(a=>{const b=c(a).parentNode;return p(b)?U:g(b)});b.wbg.__wbg_childNodes_a5762b4b3e073cf6=(a=>{const b=c(a).childNodes;return g(b)});b.wbg.__wbg_previousSibling_ef843c512fac0d77=(a=>{const b=c(a).previousSibling;return p(b)?U:g(b)});b.wbg.__wbg_nextSibling_bafccd3347d24543=(a=>{const b=c(a).nextSibling;return p(b)?U:g(b)});b.wbg.__wbg_textContent_2f37235e13f8484b=((b,d)=>{const e=c(d).textContent;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_settextContent_3ebccdd9354e1601=((a,b,d)=>{var e=G(b,d);c(a).textContent=e});b.wbg.__wbg_appendChild_d30e6b83791d04c0=function(){return F(((a,b)=>{const d=c(a).appendChild(c(b));return g(d)}),arguments)};b.wbg.__wbg_cloneNode_405d5ea3f7e0098a=function(){return F((a=>{const b=c(a).cloneNode();return g(b)}),arguments)};b.wbg.__wbg_key_a6c26b8eda8cd080=((b,d)=>{const e=c(d).key;var f=p(e)?U:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_add_e0f3c5b6e421c311=function(){return F(((a,b,d)=>{var e=G(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_remove_c6ba26a0a6906129=function(){return F(((a,b,d)=>{var e=G(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_new_7a20246daa6eec7e=function(){return F((()=>{const a=new Headers();return g(a)}),arguments)};b.wbg.__wbg_set_27f236f6d7a28c29=function(){return F(((a,b,d,e,f)=>{var g=G(b,d);var h=G(e,f);c(a).set(g,h)}),arguments)};b.wbg.__wbg_keyCode_48fe24f81bbcf215=(a=>{const b=c(a).keyCode;return b});b.wbg.__wbg_length_f845c1c304d9837a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_item_2daf9593d1a96476=((a,b)=>{const d=c(a).item(b>>>U);return p(d)?U:g(d)});b.wbg.__wbg_getItem_5395a7e200c31e89=function(){return F(((b,d,e,f)=>{var g=G(e,f);const h=c(d).getItem(g);var i=p(h)?U:o(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;r()[b/$+ Q]=j;r()[b/$+ U]=i}),arguments)};b.wbg.__wbg_removeItem_c84f914587f36b1a=function(){return F(((a,b,d)=>{var e=G(b,d);c(a).removeItem(e)}),arguments)};b.wbg.__wbg_setItem_3786c4c8dd0c9bd0=function(){return F(((a,b,d,e,f)=>{var g=G(b,d);var h=G(e,f);c(a).setItem(g,h)}),arguments)};b.wbg.__wbg_new_bc66a7e94d71957f=function(){return F((()=>{const a=new URLSearchParams();return g(a)}),arguments)};b.wbg.__wbg_newwithstrsequencesequence_f621f5f86b3f46d9=function(){return F((a=>{const b=new URLSearchParams(c(a));return g(b)}),arguments)};b.wbg.__wbg_error_e60eff06f24ab7a4=(a=>{console.error(c(a))});b.wbg.__wbg_log_a4530b4fe289336f=(a=>{console.log(c(a))});b.wbg.__wbg_warn_f260f49434e45e62=(a=>{console.warn(c(a))});b.wbg.__wbg_instanceof_WorkerGlobalScope_af28ee97555db40a=(a=>{let b;try{b=c(a) instanceof WorkerGlobalScope}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_fetch_693453ca3f88c055=((a,b)=>{const d=c(a).fetch(c(b));return g(d)});b.wbg.__wbg_instanceof_HtmlAnchorElement_76fafcefedd51299=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_target_b68f65aba6338cfb=((b,d)=>{const e=c(d).target;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_href_829df0adc5a7228a=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_target_52ddf6955f636bf5=(a=>{const b=c(a).target;return p(b)?U:g(b)});b.wbg.__wbg_defaultPrevented_ae7d433108dd159d=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_cancelBubble_976cfdf7ac449a6c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_12a068e57a98cf90=(a=>{const b=c(a).composedPath();return g(b)});b.wbg.__wbg_preventDefault_7f821f72e7c6b5d4=(a=>{c(a).preventDefault()});b.wbg.__wbg_stopPropagation_b7a931152e09c2ab=(a=>{c(a).stopPropagation()});b.wbg.__wbg_addEventListener_9bf60ea8a362e5e4=function(){return F(((a,b,d,e)=>{var f=G(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_374cbfd2bbc19ccf=function(){return F(((a,b,d,e,f)=>{var g=G(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_dispatchEvent_40c3472e9e4dcf5e=function(){return F(((a,b)=>{const d=c(a).dispatchEvent(c(b));return d}),arguments)};b.wbg.__wbg_removeEventListener_66ee1536a0b32c11=function(){return F(((a,b,d,e)=>{var f=G(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_removeEventListener_70ee8cc1640c97d7=function(){return F(((a,b,d,e,f)=>{var g=G(b,d);c(a).removeEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_new_8b18a325932736b8=function(){return F((()=>{const a=new Range();return g(a)}),arguments)};b.wbg.__wbg_deleteContents_08069ffe080b9480=function(){return F((a=>{c(a).deleteContents()}),arguments)};b.wbg.__wbg_setEndBefore_2fcd1d853bf5ebfa=function(){return F(((a,b)=>{c(a).setEndBefore(c(b))}),arguments)};b.wbg.__wbg_setStartBefore_5a200b7348513263=function(){return F(((a,b)=>{c(a).setStartBefore(c(b))}),arguments)};b.wbg.__wbg_url_d64448346abf0f74=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_newwithstr_8aa8479760b1e560=function(){return F(((a,b)=>{var c=G(a,b);const d=new Request(c);return g(d)}),arguments)};b.wbg.__wbg_newwithstrandinit_f581dff0d19a8b03=function(){return F(((a,b,d)=>{var e=G(a,b);const f=new Request(e,c(d));return g(f)}),arguments)};b.wbg.__wbg_instanceof_Response_4c3b1446206114d1=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_url_83a6a4f65f7a2b38=((b,d)=>{const e=c(d).url;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbg_redirected_8fe7aaf7e40a1256=(a=>{const b=c(a).redirected;return b});b.wbg.__wbg_setdata_86ad1e8da020aa68=((a,b,d)=>{var e=G(b,d);c(a).data=e});b.wbg.__wbg_instanceof_HtmlFormElement_7d89e65c39841f5c=(a=>{let b;try{b=c(a) instanceof HTMLFormElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_ShadowRoot_0bd39e89ab117f86=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_09eee5e3d9cf59a1=(a=>{const b=c(a).host;return g(b)});b.wbg.__wbg_matches_4cc0ff05af669dc3=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_ctrlKey_643b17aaac67db50=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_8fb7301f56e7e01c=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_c6c2a7e797d9a669=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_2a8dbd51a3f59e9c=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_button_cd87b6dabbde9631=(a=>{const b=c(a).button;return b});b.wbg.__wbg_queueMicrotask_4d890031a6a5a50c=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_adae4bc085237231=(a=>{const b=c(a).queueMicrotask;return g(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbg_get_f01601b5a68d10e3=((a,b)=>{const d=c(a)[b>>>U];return g(d)});b.wbg.__wbg_length_1009b1af0c481d7b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_c62ea9419c21fbac=((a,b)=>{var c=G(a,b);const d=new Function(c);return g(d)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==P;return d});b.wbg.__wbg_next_9b877f231f476d01=(a=>{const b=c(a).next;return g(b)});b.wbg.__wbg_next_6529ee0cca8d57ed=function(){return F((a=>{const b=c(a).next();return g(b)}),arguments)};b.wbg.__wbg_done_5fe336b092d60cf2=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_0c248a78fdc8e19f=(a=>{const b=c(a).value;return g(b)});b.wbg.__wbg_iterator_db7ca081358d4fb2=(()=>{const a=Symbol.iterator;return g(a)});b.wbg.__wbg_get_7b48513de5dc5ea4=function(){return F(((a,b)=>{const d=a0.get(c(a),c(b));return g(d)}),arguments)};b.wbg.__wbg_call_90c26b09837aba1c=function(){return F(((a,b)=>{const d=c(a).call(c(b));return g(d)}),arguments)};b.wbg.__wbg_new_9fb8d994e1c0aaac=(()=>{const a=new a1();return g(a)});b.wbg.__wbg_self_f0e34d89f33b99fd=function(){return F((()=>{const a=self.self;return g(a)}),arguments)};b.wbg.__wbg_window_d3b084224f4774d7=function(){return F((()=>{const a=window.window;return g(a)}),arguments)};b.wbg.__wbg_globalThis_9caa27ff917c6860=function(){return F((()=>{const a=globalThis.globalThis;return g(a)}),arguments)};b.wbg.__wbg_global_35dfdd59a4da3e74=function(){return F((()=>{const a=global.global;return g(a)}),arguments)};b.wbg.__wbg_decodeURI_1e508fc8ed99cae7=function(){return F(((a,b)=>{var c=G(a,b);const d=decodeURI(c);return g(d)}),arguments)};b.wbg.__wbg_encodeURI_ef679c361ea19cd8=((a,b)=>{var c=G(a,b);const d=encodeURI(c);return g(d)});b.wbg.__wbg_isArray_74fb723e24f76012=(a=>{const b=Z(c(a));return b});b.wbg.__wbg_instanceof_Error_31ca8d97f188bfbc=(a=>{let b;try{b=c(a) instanceof T}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_message_55b9ea8030688597=(a=>{const b=c(a).message;return g(b)});b.wbg.__wbg_name_e5eede664187fed6=(a=>{const b=c(a).name;return g(b)});b.wbg.__wbg_toString_a44236e90224e279=(a=>{const b=c(a).toString();return g(b)});b.wbg.__wbg_call_5da1969d7cd31ccd=function(){return F(((a,b,d)=>{const e=c(a).call(c(b),c(d));return g(e)}),arguments)};b.wbg.__wbg_getTime_9272be78826033e1=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_new0_622c21a64f3d83ea=(()=>{const a=new a2();return g(a)});b.wbg.__wbg_now_096aa89623f72d50=(()=>{const a=a2.now();return a});b.wbg.__wbg_is_ff7acd231c75c0e4=((a,b)=>{const d=a1.is(c(a),c(b));return d});b.wbg.__wbg_toString_6577cc00288ad588=(a=>{const b=c(a).toString();return g(b)});b.wbg.__wbg_exec_42513e2d2ddabd95=((a,b,d)=>{var e=G(b,d);const f=c(a).exec(e);return p(f)?U:g(f)});b.wbg.__wbg_new_e145ee1b0ed9b4aa=((a,b,c,d)=>{var e=G(a,b);var f=G(c,d);const h=new RegExp(e,f);return g(h)});b.wbg.__wbg_resolve_6e1c6553a82f85b7=(a=>{const b=Promise.resolve(c(a));return g(b)});b.wbg.__wbg_then_3ab08cd4fbb91ae9=((a,b)=>{const d=c(a).then(c(b));return g(d)});b.wbg.__wbg_then_8371cc12cfedc5a2=((a,b,d)=>{const e=c(a).then(c(b),c(d));return g(e)});b.wbg.__wbg_set_759f75cd92b612d2=function(){return F(((a,b,d)=>{const e=a0.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/$+ Q]=g;r()[b/$+ U]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new T(k(a,b))});b.wbg.__wbindgen_closure_wrapper220=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper222=((a,b,c)=>{const d=v(a,b,a3,x);return g(d)});b.wbg.__wbindgen_closure_wrapper224=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper226=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper228=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper230=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper232=((a,b,c)=>{const d=v(a,b,a3,w);return g(d)});b.wbg.__wbindgen_closure_wrapper1279=((a,b,c)=>{const d=v(a,b,a4,y);return g(d)});b.wbg.__wbindgen_closure_wrapper1281=((a,b,c)=>{const d=v(a,b,a4,z);return g(d)});b.wbg.__wbindgen_closure_wrapper1494=((a,b,c)=>{const d=v(a,b,a5,A);return g(d)});b.wbg.__wbindgen_closure_wrapper1496=((a,b,c)=>{const d=B(a,b,a5,C);return g(d)});b.wbg.__wbindgen_closure_wrapper1686=((a,b,c)=>{const d=v(a,b,644,D);return g(d)});b.wbg.__wbindgen_closure_wrapper3331=((a,b,c)=>{const d=v(a,b,687,E);return g(d)});return b});var o=((a,b,c)=>{if(c===O){const c=m.encode(a);const d=b(c.length,Q)>>>U;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,Q)>>>U;const f=j();let g=U;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==U){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,Q)>>>U;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var j=(()=>{if(i===P||i.byteLength===U){i=new Uint8Array(a.memory.buffer)};return i});var H=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var C=((b,c)=>{a._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcfdf5a960a562acb(b,c)});var g=(a=>{if(d===b.length)b.push(b.length+ Q);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});var G=((a,b)=>{if(a===U){return c(b)}else{return k(a,b)}});var r=(()=>{if(q===P||q.byteLength===U){q=new Int32Array(a.memory.buffer)};return q});var A=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h41f163f5ff67a642(b,c,g(d))});var M=(async(b)=>{if(a!==O)return a;if(typeof b===R){b=new URL(`zone-ui-7fd4fcf53c155a55deb5c522f52950b3507276d33e96abcda4bb667ab8dd6098fc00860fdd3eb2e65184e4320c07b73f_bg.wasm`,import.meta.url)};const c=I();if(typeof b===Y||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};J(c);const {instance:d,module:e}=await H(await b,c);return K(d,e)});var L=(b=>{if(a!==O)return a;const c=I();J(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return K(d,b)});var K=((b,c)=>{a=b.exports;M.__wbindgen_wasm_module=c;s=P;q=P;i=P;a.__wbindgen_start();return a});var B=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=U}}};g.original=f;return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var D=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8839ac221b1445a6(b,c)});var y=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hbcf02790e8935e22(b,c)});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=U;try{return e(c,f.b,...b)}finally{if(--f.cnt===U){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var k=((a,b)=>{a=a>>>U;return h.decode(j().subarray(a,a+ b))});var w=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h17aede3c82042dcc(b,c,g(d))});let a;const b=new N(128).fill(O);b.push(O,P,!0,!1);let d=b.length;const h=typeof TextDecoder!==R?new TextDecoder(S,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw T(`TextDecoder not available`)}};if(typeof TextDecoder!==R){h.decode()};let i=P;let l=U;const m=typeof TextEncoder!==R?new TextEncoder(S):{encode:()=>{throw T(`TextEncoder not available`)}};const n=typeof m.encodeInto===V?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=P;let s=P;export default M;export{L as initSync}