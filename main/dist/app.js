const appLaunchTime = Date.now();

const transforms = {
    option1: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝑎𝑏𝑐𝑑𝑒𝑓𝑔ℎ𝑖𝑗𝑘𝑙𝑚𝑛𝑜𝑝𝑞𝑟𝑠𝑡𝑢𝑣𝑤𝑥𝑦𝑧𝐴𝐵𝐶𝐷𝐸𝐹𝐺𝐻𝐼𝐽𝐾𝐿𝑀𝑁𝑂𝑃𝑄𝑅𝑆𝑇𝑈𝑉𝑊𝑋𝑌𝑍'),
    option2: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙𝟎𝟏𝟐𝟑𝟒𝟓𝟔𝟕𝟖𝟗'),
    option3: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁'),
    option4: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝘢𝘣𝘤𝘥𝘦𝘧𝘨𝘩𝘪𝘫𝘬𝘭𝘮𝘯𝘰𝘱𝘲𝘳𝘴𝘵𝘶𝘷𝘸𝘹𝘺𝘻𝘈𝘉𝘊𝘋𝘌𝘍𝘎𝘏𝘐𝘑𝘒𝘓𝘔𝘕𝘖𝘗𝘘𝘙𝘚𝘛𝘜𝘝𝘞𝘟𝘠𝘡'),
    option5: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '𝗮𝗯𝗰𝗱𝗲𝗳𝗴𝗵𝗶𝗷𝗸𝗹𝗺𝗻𝗼𝗽𝗾𝗿𝘀𝘁𝘂𝘃𝘄𝘅𝘆𝘇𝗔𝗕𝗖𝗗𝗘𝗙𝗚𝗛𝗜𝗝𝗞𝗟𝗠𝗡𝗢𝗣𝗤𝗥𝗦𝗧𝗨𝗩𝗪𝗫𝗬𝗭𝟬𝟭𝟮𝟯𝟰𝟱𝟲𝟳𝟴𝟵'),
    option6: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝙖𝙗𝙘𝙙𝙚𝙛𝙜𝙝𝙞𝙟𝙠𝙡𝙢𝙣𝙤𝙥𝙦𝙧𝙨𝙩𝙪𝙫𝙬𝙭𝙮𝙯𝘼𝘽𝘾𝘿𝙀𝙁𝙂𝙃𝙄𝙅𝙆𝙇𝙈𝙉𝙊𝙋𝙌𝙍𝙎𝙏𝙐𝙑𝙒𝙓𝙔𝙕'),
    option7: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ𝟘𝟙𝟚𝟛𝟜𝟝𝟞𝟟𝟠𝟡'),
    option8: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝖆𝖇𝖈𝖉𝖊𝖋𝖌𝖍𝖎𝖏𝖐𝖑𝖒𝖓𝖔𝖕𝖖𝖗𝖘𝖙𝖚𝖛𝖜𝖝𝖞𝖟𝕬𝕭𝕮𝕯𝕰𝕱𝕲𝕳𝕴𝕵𝕶𝕷𝕸𝕹𝕺𝕻𝕼𝕽𝕾𝕿𝖀𝖁𝖂𝖃𝖄𝖅'),
    option9: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷𝔄𝔅ℭ𝔇𝔈𝔉𝔊ℌℑ𝔍𝔎𝔏𝔑𝔒𝔓𝔔ℛ𝔖𝔗𝔘𝔙𝔚𝔛𝔜ℨ'),
    option10: (text) => {
        const normal = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.?!',";
        const upside = "ɐqɔpǝɟƃɥᴉɾʞlɯuodbɹsʇnʌʍxʎzⱯꓭƆᗡƎℲ⅁HIᒋꞰ˥WNOԀΌᴚS⊥∩ΛMX⅄Z0⇂ᘔƐ߈59Ɫ86˙¿¡,";
        return text.split('').map(c => upside[normal.indexOf(c)] || c).reverse().join('');
    },
    option11: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', 'ᵃᵇᶜᵈᵉᶠᵍʰⁱʲᵏˡᵐⁿᵒᵖqʳˢᵗᵘᵛʷˣʸᶻᴬᴮᶜᴰᴱᶠᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾQᴿˢᵀᵁⱽᵂˣʸᶻ⁰¹²³⁴⁵⁶⁷⁸⁹'),
    option12: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉𝟶𝟷𝟸𝟹𝟺𝟻𝟼𝟽𝟾𝟿'),
    option13: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝖺𝖻𝖼𝖽𝖾𝖿𝗀𝗁𝗂𝗃𝗄𝗅𝗆𝗇𝗈𝗉𝗊𝗋𝗌𝗍𝗎𝗏𝗐𝗑𝗒𝗓𝖠𝖡𝖢𝖣𝖤𝖥𝖦𝖧𝖨𝖩𝖪𝖫𝖬𝖭𝖮𝖯𝖰𝖱𝖲𝖳𝖴𝖵𝖶𝖷𝖸𝖹'),
    option14: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyz', 'ᴀʙᴄᴅᴇꜰɢʜɪᴊᴋʟᴍɴᴏᴩqʀꜱᴛᴜᴠᴡxʏᴢ'),
    option15: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝓪𝓫𝓬𝓭𝓮𝓯𝓰𝓱𝓲𝓳𝓴𝓵𝓶𝓷𝓸𝓹𝓺𝓻𝓼𝓽𝓾𝓿𝔀𝔁𝔂𝔃𝓐𝓑𝓒𝓓𝓔𝓕𝓖𝓗𝓘𝓙𝓚𝓛𝓜𝓝𝓞𝓟𝓠𝓡𝓢𝓣𝓤𝓥𝓦𝓧𝓨𝓩'),
    option16: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ', '𝒶𝒷𝒸𝒹ℯ𝒻ℊ𝒽𝒾𝒿𝓀𝓁𝓂𝓃ℴ𝓅𝓆𝓇𝓈𝓉𝓊𝓋𝓌𝓍𝓎𝓏𝒜ℬ𝒞𝒟ℰℱ𝒢ℋℐ𝒥𝒦ℒℳ𝒩𝒪𝒫𝒬ℛ𝒮𝒯𝒰𝒱𝒲𝒳𝒴𝒵'),
    option17: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '🅰🅱🅲🅳🅴🅵🅶🅷🅸🅹🅺🅻🅼🅽🅾🅿🆀🆁🆂🆃🆄🆅🆆🆇🆈🆉🅰🅱🅲🅳🅴🅵🅶🅷🅸🅹🅺🅻🅼🅽🅾🅿🆀🆁🆂🆃🆄🆅🆆🆇🆈🆉0123456789'),
    option18: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '🄰🄱🄲🄳🄴🄵🄶🄷🄸🄹🄺🄻🄼🄽🄾🄿🅀🅁🅂🅃🅄🅅🅆🅇🅈🅉🄰🄱🄲🄳🄴🄵🄶🄷🄸🄹🄺🄻🄼🄽🄾🄿🅀🅁🅂🅃🅄🅅🅆🅇🅈🅉0⃣1⃣2⃣3⃣4⃣5⃣6⃣7⃣8⃣9⃣'),
    option19: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', '🅐🅑🅒🅓🅔🅕🅖🅗🅘🅙🅚🅛🅜🅝🅞🅟🅠🅡🅢🅣🅤🅥🅦🅧🅨🅩🅐🅑🅒🅓🅔🅕🅖🅗🅘🅙🅚🅛🅜🅝🅞🅟🅠🅡🅢🅣🅤🅥🅦🅧🅨🅩🄌➊➋➌➍➎➏➐➑➒'),
    option20: (text) => transformMap(text, 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789', 'ⓐⓑⓒⓓⓔⓕⓖⓗⓘⓙⓚⓛⓜⓝⓞⓟⓠⓡⓢⓣⓤⓥⓦⓧⓨⓩⒶⒷⒸⒹⒺⒻⒼⒽⒾⒿⓀⓁⓂⓃⓄⓅⓆⓇⓈⓉⓊⓋⓌⓍⓎⓏ⓪①②③④⑤⑥⑦⑧⑨')
};

const graphemeSegmenter = new Intl.Segmenter(undefined, { granularity: 'grapheme' });
function toGraphemes(str) {
    return Array.from(graphemeSegmenter.segment(str), (s) => s.segment);
}

function transformMap(text, alphabetFrom, alphabetTo) {
    const fromArr = toGraphemes(alphabetFrom);
    const toArr = toGraphemes(alphabetTo);
    return toGraphemes(text).map(char => {
        const index = fromArr.indexOf(char);
        return index !== -1 && toArr[index] ? toArr[index] : char;
    }).join('');
}


function clamp(n, min, max) { return Math.min(Math.max(n, min), max); }
function toHex(n) { return clamp(Math.round(n), 0, 255).toString(16).padStart(2, '0'); }
function trimNum(n, decimals = 2) { return String(parseFloat(n.toFixed(decimals))); }
function toShorthandDigit(n) { return clamp(Math.round(n / 17), 0, 15).toString(16); }

function hexToRgba(hex) {
    hex = hex.replace('#', '');
    if (hex.length === 3) hex = hex.split('').map(c => c + c).join('') + 'ff';
    else if (hex.length === 4) hex = hex.split('').map(c => c + c).join('');
    else if (hex.length === 6) hex += 'ff';
    else if (hex.length !== 8) return null;
    return {
        r: parseInt(hex.slice(0, 2), 16),
        g: parseInt(hex.slice(2, 4), 16),
        b: parseInt(hex.slice(4, 6), 16),
        a: parseInt(hex.slice(6, 8), 16) / 255
    };
}

function hslToRgb(h, s, l) {
    h = ((h % 360) + 360) % 360; s /= 100; l /= 100;
    const c = (1 - Math.abs(2 * l - 1)) * s;
    const x = c * (1 - Math.abs((h / 60) % 2 - 1));
    const m = l - c / 2;
    let r, g, b;
    if (h < 60) [r, g, b] = [c, x, 0];
    else if (h < 120) [r, g, b] = [x, c, 0];
    else if (h < 180) [r, g, b] = [0, c, x];
    else if (h < 240) [r, g, b] = [0, x, c];
    else if (h < 300) [r, g, b] = [x, 0, c];
    else [r, g, b] = [c, 0, x];
    return { r: (r + m) * 255, g: (g + m) * 255, b: (b + m) * 255 };
}
function rgbToHsl(r, g, b) {
    r /= 255; g /= 255; b /= 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    let h = 0; const l = (max + min) / 2; const d = max - min;
    let s = 0;
    if (d !== 0) {
        s = d / (1 - Math.abs(2 * l - 1));
        switch (max) {
            case r: h = 60 * (((g - b) / d) % 6); break;
            case g: h = 60 * ((b - r) / d + 2); break;
            case b: h = 60 * ((r - g) / d + 4); break;
        }
    }
    if (h < 0) h += 360;
    return { h: Math.round(h), s: Math.round(s * 100), l: Math.round(l * 100) };
}

function hsvToRgb(h, s, v) {
    h = ((h % 360) + 360) % 360; s /= 100; v /= 100;
    const c = v * s; const x = c * (1 - Math.abs((h / 60) % 2 - 1)); const m = v - c;
    let r, g, b;
    if (h < 60) [r, g, b] = [c, x, 0];
    else if (h < 120) [r, g, b] = [x, c, 0];
    else if (h < 180) [r, g, b] = [0, c, x];
    else if (h < 240) [r, g, b] = [0, x, c];
    else if (h < 300) [r, g, b] = [x, 0, c];
    else [r, g, b] = [c, 0, x];
    return { r: (r + m) * 255, g: (g + m) * 255, b: (b + m) * 255 };
}
function rgbToHsv(r, g, b) {
    r /= 255; g /= 255; b /= 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    const d = max - min;
    let h = 0;
    if (d !== 0) {
        switch (max) {
            case r: h = 60 * (((g - b) / d) % 6); break;
            case g: h = 60 * ((b - r) / d + 2); break;
            case b: h = 60 * ((r - g) / d + 4); break;
        }
    }
    if (h < 0) h += 360;
    const s = max === 0 ? 0 : d / max;
    return { h: Math.round(h), s: Math.round(s * 100), v: Math.round(max * 100) };
}

function cmykToRgb(c, m, y, k) {
    c /= 100; m /= 100; y /= 100; k /= 100;
    return { r: 255 * (1 - c) * (1 - k), g: 255 * (1 - m) * (1 - k), b: 255 * (1 - y) * (1 - k) };
}
function rgbToCmyk(r, g, b) {
    r /= 255; g /= 255; b /= 255;
    const k = 1 - Math.max(r, g, b);
    if (k >= 1) return { c: 0, m: 0, y: 0, k: 100 };
    const c = (1 - r - k) / (1 - k), m = (1 - g - k) / (1 - k), y = (1 - b - k) / (1 - k);
    return { c: Math.round(c * 100), m: Math.round(m * 100), y: Math.round(y * 100), k: Math.round(k * 100) };
}

function srgbToLinear(c) { c /= 255; return c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4); }
function linearToSrgb(c) { const v = c <= 0.0031308 ? c * 12.92 : 1.055 * Math.pow(Math.max(c, 0), 1 / 2.4) - 0.055; return v * 255; }

function rgbToOklab(r, g, b) {
    const lr = srgbToLinear(r), lg = srgbToLinear(g), lb = srgbToLinear(b);
    const l = 0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb;
    const m = 0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb;
    const s = 0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb;
    const l_ = Math.cbrt(l), m_ = Math.cbrt(m), s_ = Math.cbrt(s);
    return {
        L: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_
    };
}
function oklabToRgb(L, a, b) {
    const l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    const m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    const s_ = L - 0.0894841775 * a - 1.2914855480 * b;
    const l = l_ ** 3, m = m_ ** 3, s = s_ ** 3;
    const lr = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    const lg = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    const lb = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;
    return { r: linearToSrgb(lr), g: linearToSrgb(lg), b: linearToSrgb(lb) };
}
function rgbToOklch(r, g, b) {
    const { L, a, b: bb } = rgbToOklab(r, g, b);
    const C = Math.sqrt(a * a + bb * bb);
    let H = Math.atan2(bb, a) * 180 / Math.PI;
    if (H < 0) H += 360;
    return { L, C, H };
}
function oklchToRgb(L, C, H) {
    const rad = H * Math.PI / 180;
    return oklabToRgb(L, C * Math.cos(rad), C * Math.sin(rad));
}

function rgbToXyz(r, g, b) {
    const lr = srgbToLinear(r), lg = srgbToLinear(g), lb = srgbToLinear(b);
    return {
        x: 0.4124564 * lr + 0.3575761 * lg + 0.1804375 * lb,
        y: 0.2126729 * lr + 0.7151522 * lg + 0.0721750 * lb,
        z: 0.0193339 * lr + 0.1191920 * lg + 0.9503041 * lb
    };
}
function xyzToRgb(x, y, z) {
    const lr = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    const lg = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    const lb = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    return { r: linearToSrgb(lr), g: linearToSrgb(lg), b: linearToSrgb(lb) };
}

const D65 = { x: 0.95047, y: 1.0, z: 1.08883 };
function fLab(t) {
    const e = 216 / 24389, k = 24389 / 27;
    return t > e ? Math.cbrt(t) : (k * t + 16) / 116;
}
function fLabInv(t) {
    const e = 6 / 29;
    return t > e ? t ** 3 : 3 * e * e * (t - 4 / 29);
}

function rgbToLch(r, g, b) {
    const { x, y, z } = rgbToXyz(r, g, b);
    const fx = fLab(x / D65.x), fy = fLab(y / D65.y), fz = fLab(z / D65.z);
    const L = 116 * fy - 16;
    const a = 500 * (fx - fy);
    const bb = 200 * (fy - fz);
    const C = Math.sqrt(a * a + bb * bb);
    let H = Math.atan2(bb, a) * 180 / Math.PI;
    if (H < 0) H += 360;
    return { L, C, H };
}
function lchToRgb(L, C, H) {
    const rad = H * Math.PI / 180;
    const a = C * Math.cos(rad);
    const bb = C * Math.sin(rad);
    const fy = (L + 16) / 116;
    const fx = fy + a / 500;
    const fz = fy - bb / 200;
    const x = D65.x * fLabInv(fx);
    const y = D65.y * fLabInv(fy);
    const z = D65.z * fLabInv(fz);
    return xyzToRgb(x, y, z);
}

const namedColors = {
    aliceblue:'#f0f8ff', antiquewhite:'#faebd7', aqua:'#00ffff', aquamarine:'#7fffd4', azure:'#f0ffff',
    beige:'#f5f5dc', bisque:'#ffe4c4', black:'#000000', blanchedalmond:'#ffebcd', blue:'#0000ff',
    blueviolet:'#8a2be2', brown:'#a52a2a', burlywood:'#deb887', cadetblue:'#5f9ea0', chartreuse:'#7fff00',
    chocolate:'#d2691e', coral:'#ff7f50', cornflowerblue:'#6495ed', cornsilk:'#fff8dc', crimson:'#dc143c',
    cyan:'#00ffff', darkblue:'#00008b', darkcyan:'#008b8b', darkgoldenrod:'#b8860b', darkgray:'#a9a9a9',
    darkgreen:'#006400', darkgrey:'#a9a9a9', darkkhaki:'#bdb76b', darkmagenta:'#8b008b', darkolivegreen:'#556b2f',
    darkorange:'#ff8c00', darkorchid:'#9932cc', darkred:'#8b0000', darksalmon:'#e9967a', darkseagreen:'#8fbc8f',
    darkslateblue:'#483d8b', darkslategray:'#2f4f4f', darkslategrey:'#2f4f4f', darkturquoise:'#00ced1', darkviolet:'#9400d3',
    deeppink:'#ff1493', deepskyblue:'#00bfff', dimgray:'#696969', dimgrey:'#696969', dodgerblue:'#1e90ff',
    firebrick:'#b22222', floralwhite:'#fffaf0', forestgreen:'#228b22', fuchsia:'#ff00ff', gainsboro:'#dcdcdc',
    ghostwhite:'#f8f8ff', gold:'#ffd700', goldenrod:'#daa520', gray:'#808080', green:'#008000',
    greenyellow:'#adff2f', grey:'#808080', honeydew:'#f0fff0', hotpink:'#ff69b4', indianred:'#cd5c5c',
    indigo:'#4b0082', ivory:'#fffff0', khaki:'#f0e68c', lavender:'#e6e6fa', lavenderblush:'#fff0f5',
    lawngreen:'#7cfc00', lemonchiffon:'#fffacd', lightblue:'#add8e6', lightcoral:'#f08080', lightcyan:'#e0ffff',
    lightgoldenrodyellow:'#fafad2', lightgray:'#d3d3d3', lightgreen:'#90ee90', lightgrey:'#d3d3d3', lightpink:'#ffb6c1',
    lightsalmon:'#ffa07a', lightseagreen:'#20b2aa', lightskyblue:'#87cefa', lightslategray:'#778899', lightslategrey:'#778899',
    lightsteelblue:'#b0c4de', lightyellow:'#ffffe0', lime:'#00ff00', limegreen:'#32cd32', linen:'#faf0e6',
    magenta:'#ff00ff', maroon:'#800000', mediumaquamarine:'#66cdaa', mediumblue:'#0000cd', mediumorchid:'#ba55d3',
    mediumpurple:'#9370db', mediumseagreen:'#3cb371', mediumslateblue:'#7b68ee', mediumspringgreen:'#00fa9a', mediumturquoise:'#48d1cc',
    mediumvioletred:'#c71585', midnightblue:'#191970', mintcream:'#f5fffa', mistyrose:'#ffe4e1', moccasin:'#ffe4b5',
    navajowhite:'#ffdead', navy:'#000080', oldlace:'#fdf5e6', olive:'#808000', olivedrab:'#6b8e23',
    orange:'#ffa500', orangered:'#ff4500', orchid:'#da70d6', palegoldenrod:'#eee8aa', palegreen:'#98fb98',
    paleturquoise:'#afeeee', palevioletred:'#db7093', papayawhip:'#ffefd5', peachpuff:'#ffdab9', peru:'#cd853f',
    pink:'#ffc0cb', plum:'#dda0dd', powderblue:'#b0e0e6', purple:'#800080', rebeccapurple:'#663399',
    red:'#ff0000', rosybrown:'#bc8f8f', royalblue:'#4169e1', saddlebrown:'#8b4513', salmon:'#fa8072',
    sandybrown:'#f4a460', seagreen:'#2e8b57', seashell:'#fff5ee', sienna:'#a0522d', silver:'#c0c0c0',
    skyblue:'#87ceeb', slateblue:'#6a5acd', slategray:'#708090', slategrey:'#708090', snow:'#fffafa',
    springgreen:'#00ff7f', steelblue:'#4682b4', tan:'#d2b48c', teal:'#008080', thistle:'#d8bfd8',
    tomato:'#ff6347', turquoise:'#40e0d0', violet:'#ee82ee', wheat:'#f5deb3', white:'#ffffff',
    whitesmoke:'#f5f5f5', yellow:'#ffff00', yellowgreen:'#9acd32'
};

function nearestNamedColor(r, g, b) {
    let best = 'black', bestDist = Infinity;
    for (const name in namedColors) {
        const c = hexToRgba(namedColors[name]);
        const d = (r - c.r) ** 2 + (g - c.g) ** 2 + (b - c.b) ** 2;
        if (d < bestDist) { bestDist = d; best = name; }
    }
    return best;
}

function parsePercentOr255(v) { return v.endsWith('%') ? (parseFloat(v) / 100) * 255 : parseFloat(v); }
function parseAlpha(v) { return v.endsWith('%') ? parseFloat(v) / 100 : parseFloat(v); }


const colorParsers = [
    {
        re: /^rgba?\(\s*([\d.]+%?)\s*[, ]\s*([\d.]+%?)\s*[, ]\s*([\d.]+%?)\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => ({ r: parsePercentOr255(m[1]), g: parsePercentOr255(m[2]), b: parsePercentOr255(m[3]), a: m[4] !== undefined ? parseAlpha(m[4]) : 1 })
    },
    {
        re: /^hsla?\(\s*([\d.]+)(?:deg)?\s*[, ]\s*([\d.]+)%\s*[, ]\s*([\d.]+)%\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = hslToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    {
        re: /^hs[vb]\(\s*([\d.]+)(?:deg)?\s*[, ]\s*([\d.]+)%\s*[, ]\s*([\d.]+)%\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = hsvToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    {
        re: /^cmyk\(\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*\)$/i,
        parse: (m) => cmykToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3]), parseFloat(m[4]))
    },
    {
        re: /^oklch\(\s*([\d.]+)%?\s+([\d.]+)\s+([\d.]+)(?:deg)?\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = oklchToRgb(parseFloat(m[1]) / 100, parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    {
        re: /^oklab\(\s*([\d.]+)%?\s+(-?[\d.]+)\s+(-?[\d.]+)\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = oklabToRgb(parseFloat(m[1]) / 100, parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    {
        re: /^lch\(\s*([\d.]+)%?\s+([\d.]+)\s+([\d.]+)(?:deg)?\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = lchToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    {
        re: /UIColor\(\s*red:\s*([\d.]+),?\s*green:\s*([\d.]+),?\s*blue:\s*([\d.]+)(?:,?\s*alpha:\s*([\d.]+))?\s*\)/i,
        parse: (m) => ({ r: parseFloat(m[1]) * 255, g: parseFloat(m[2]) * 255, b: parseFloat(m[3]) * 255, a: m[4] !== undefined ? parseFloat(m[4]) : 1 })
    },
    {
        re: /(?:Color\(\s*0x|0x)([0-9a-f]{8})\)?/i,
        parse: (m) => { const h = m[1]; return { r: parseInt(h.slice(2, 4), 16), g: parseInt(h.slice(4, 6), 16), b: parseInt(h.slice(6, 8), 16), a: parseInt(h.slice(0, 2), 16) / 255 }; }
    },
    {
        re: /Color\.parseColor\(\s*"(#?[0-9a-f]{3,8})"\s*\)/i,
        parse: (m) => hexToRgba(m[1])
    },
    {
        re: /^#([0-9a-f]{3,8})$/i,
        parse: (m) => hexToRgba(m[1])
    },
    {
        re: /^[a-z]+$/i,
        parse: (m, raw) => { const hex = namedColors[raw.toLowerCase()]; return hex ? hexToRgba(hex) : null; }
    },
    {
        re: /^([\d.]+%?)[\s,]+([\d.]+%?)[\s,]+([\d.]+%?)(?:[\s,]+([\d.]+%?))?$/,
        parse: (m) => ({ r: parsePercentOr255(m[1]), g: parsePercentOr255(m[2]), b: parsePercentOr255(m[3]), a: m[4] !== undefined ? parseAlpha(m[4]) : 1 })
    },
    {
        re: /^([0-9a-f]{3}|[0-9a-f]{4}|[0-9a-f]{6}|[0-9a-f]{8})$/i,
        parse: (m) => hexToRgba(m[1])
    }
];

function parseColor(raw) {
    raw = raw.trim();
    if (!raw) return null;
    for (const parser of colorParsers) {
        const m = raw.match(parser.re);
        if (m) {
            const result = parser.parse(m, raw);
            if (result && isFinite(result.r) && isFinite(result.g) && isFinite(result.b)) {
                return {
                    r: clamp(Math.round(result.r), 0, 255),
                    g: clamp(Math.round(result.g), 0, 255),
                    b: clamp(Math.round(result.b), 0, 255),
                    a: clamp(result.a === undefined ? 1 : result.a, 0, 1)
                };
            }
        }
    }
    return null;
}


function formatColor({ r, g, b, a }, option) {
    switch (option) {
        case 'option1':
            return `#${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
        case 'option2':
            return `#${toShorthandDigit(r)}${toShorthandDigit(g)}${toShorthandDigit(b)}`.toUpperCase();
        case 'option3':
            return `#${toHex(r)}${toHex(g)}${toHex(b)}${toHex(Math.round(a * 255))}`.toUpperCase();
        case 'option4':
            return `#${toShorthandDigit(r)}${toShorthandDigit(g)}${toShorthandDigit(b)}${toShorthandDigit(a * 255)}`.toUpperCase();
        case 'option5':
            return `rgb(${r}, ${g}, ${b})`;
        case 'option6': {
            const pr = Math.round((r / 255) * 100), pg = Math.round((g / 255) * 100), pb = Math.round((b / 255) * 100);
            return `rgb(${pr}%, ${pg}%, ${pb}%)`;
        }
        case 'option7':
            return `rgba(${r}, ${g}, ${b}, ${trimNum(a)})`;
        case 'option8': {
            const pr = Math.round((r / 255) * 100), pg = Math.round((g / 255) * 100), pb = Math.round((b / 255) * 100);
            return `rgba(${pr}%, ${pg}%, ${pb}%, ${trimNum(a)})`;
        }
        case 'option9': {
            const { h, s, l } = rgbToHsl(r, g, b);
            return `hsl(${h}, ${s}%, ${l}%)`;
        }
        case 'option10': {
            const { h, s, l } = rgbToHsl(r, g, b);
            return `hsla(${h}, ${s}%, ${l}%, ${trimNum(a)})`;
        }
        case 'option11': {
            const { h, s, v } = rgbToHsv(r, g, b);
            return `hsv(${h}, ${s}%, ${v}%)`;
        }
        case 'option12': {
            const { c, m, y, k } = rgbToCmyk(r, g, b);
            return `cmyk(${c}%, ${m}%, ${y}%, ${k}%)`;
        }
        case 'option13':
            return nearestNamedColor(r, g, b);
        case 'option14':
            return `UIColor(red: ${(r / 255).toFixed(3)}, green: ${(g / 255).toFixed(3)}, blue: ${(b / 255).toFixed(3)}, alpha: ${a.toFixed(3)})`;
        case 'option15':
            return `0x${toHex(Math.round(a * 255))}${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
        case 'option16':
            return `Color(0x${toHex(Math.round(a * 255))}${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase() + ')';
        case 'option17': {
            const { L, C, H } = rgbToOklch(r, g, b);
            const Lp = (L * 100).toFixed(2);
            return a < 1 ? `oklch(${Lp}% ${C} ${H} / ${trimNum(a)})` : `oklch(${Lp}% ${C} ${H})`;
        }
        case 'option18': {
            const { L, C, H } = rgbToLch(r, g, b);
            const Lp = L.toFixed(2);
            return a < 1 ? `lch(${Lp}% ${C} ${H} / ${trimNum(a)})` : `lch(${Lp}% ${C} ${H})`;
        }
        default:
            return `#${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
    }
}

const ALPHA_AWARE_FORMATS = new Set([
    'option3', 'option4',
    'option7', 'option8',
    'option10',
    'option14',
    'option15', 'option16',
    'option17', 'option18'
]);


const warnBox = document.querySelector('.warn');
const warnItems = new Map();

warnBox.querySelectorAll('a[id]').forEach((el) => {
    el.dataset.defaultText = el.textContent;
    warnItems.set(el.id, el);
});

warnBox.addEventListener('animationend', (e) => {
    if (e.animationName === 'warn-out') {
        warnBox.classList.remove('show', 'hide');
    }
});

function triggerWarn(id, message) {
    if (appSettings.hideWarnNotifications) return;

    const activeEl = warnItems.get(id);
    if (!activeEl) return;

    let text = activeEl.dataset.defaultText;
    if (message) {
        const custom = String(message);
        text = custom.length > 140 ? `${custom.slice(0, 140)}…` : custom;
    }
    activeEl.textContent = text;

    warnItems.forEach((el) => el.classList.toggle('active', el === activeEl));

    warnBox.classList.remove('show', 'hide');
    void warnBox.offsetWidth;
    warnBox.classList.add('show');

    clearTimeout(warnBox.warnTimeout);
    warnBox.warnTimeout = setTimeout(() => {
        warnBox.classList.remove('show');
        warnBox.classList.add('hide');
    }, 2800);
}

async function doCopyValid(text) {
    if (!text) {
        triggerWarn('copy-empty');
        return;
    }
    try {
        await navigator.clipboard.writeText(text);
    } catch (err) {
        const textArea = document.createElement("textarea");
        textArea.value = text;
        document.body.appendChild(textArea);
        textArea.select();
        document.execCommand("copy");
        document.body.removeChild(textArea);
    }

    triggerWarn('copy-valid');
}

function doCopyInvalid() {
    triggerWarn('copy-invalid');
}

function doFileInvalid() {
    triggerWarn('file-invalid');
}

function doFileSaved() {
    triggerWarn('file-saved');
}

function doFileCorrupted(message) {
    triggerWarn('file-corrupted', message);
}

function doMediaConvertInvalid() {
    triggerWarn('media-convert-invalid');
}

function doFileNameEmpty() {
    triggerWarn('file-name-empty');
}

function doFileNotAttached() {
    triggerWarn('file-not-attached');
}


const SETTINGS_KEY = 'ftools:media-converter-settings';

async function loadSettings() {
    try {
        const raw = localStorage.getItem(SETTINGS_KEY);
        return raw ? JSON.parse(raw) : {};
    } catch (err) {
        console.error('Could not load settings:', err);
        return {};
    }
}

async function saveSettings(settings) {
    try {
        localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
    } catch (err) {
        console.error('Could not save settings:', err);
    }
}

const APP_SETTINGS_KEY = 'ftools:app-settings';

async function loadAppSettings() {
    try {
        const raw = localStorage.getItem(APP_SETTINGS_KEY);
        return raw ? JSON.parse(raw) : {};
    } catch (err) {
        console.error('Could not load app settings:', err);
        return {};
    }
}

async function saveAppSettings(settings) {
    try {
        localStorage.setItem(APP_SETTINGS_KEY, JSON.stringify(settings));
    } catch (err) {
        console.error('Could not save app settings:', err);
    }
}

let appSettings = {
    alwaysOnTop: false,
    rememberLastTool: false,
    closeOnFocusLoss: false,
    hideWarnNotifications: false,
    lastTool: 'fontstyler',
    appTheme: 'dark',
    highContrast: false
};


function setupFontStyler() {
    const panel = document.getElementById('fontstyler');
    if (!panel) return;
    const inputBox = panel.querySelector('.input-box');
    const dropdown = panel.querySelector('.dropdown');
    const outputDiv = panel.querySelector('.output-box .content');
    const copyButton = panel.querySelector('#copy.button');

    function process() {
        const text = inputBox.value;
        const selectedOption = dropdown.value;
        if (!text) { outputDiv.textContent = ""; return; }
        outputDiv.textContent = transforms[selectedOption] ? transforms[selectedOption](text) : text;
    }

    inputBox.addEventListener('input', process);
    dropdown.addEventListener('change', process);
    process();

    copyButton.addEventListener('click', () => doCopyValid(outputDiv.textContent));
}


const pickerBox = document.getElementById('picker-box');
const pickerHandle = document.getElementById('picker-handle');
const pickerDarkness = document.getElementById('darkness');
const pickerTransparency = document.getElementById('transparency');
const pickerPreview = document.querySelector('#colorpicker .color-preview');

let pickerPosXPct = 0;
let pickerPosYPct = 0;

function updatePickerPreview(hue, lightness) {
    const darkValue = parseFloat(pickerDarkness.value) / 100;
    const alphaValue = 1 - parseFloat(pickerTransparency.value) / 100;
    const baseHsl = `hsl(${hue}, 100%, ${lightness}%)`;
    const darkMix = `color-mix(in srgb, ${baseHsl} ${(1 - darkValue) * 100}%, black ${darkValue * 100}%)`;
    const finalColor = `color-mix(in srgb, ${darkMix} ${alphaValue * 100}%, transparent ${(1 - alphaValue) * 100}%)`;
    pickerPreview.style.backgroundColor = finalColor;
    pickerDarkness.style.background = `linear-gradient(${baseHsl}, #0000)`;
    pickerTransparency.style.background = `linear-gradient(${darkMix}, #0000)`;
}

function pickerCurrentHue() {
    return (pickerPosXPct / 100) * 360;
}

function pickerCurrentLightness() {
    return 50 + (pickerPosYPct / 100) * 50;
}

function applyPickerPosition(xPct, yPct) {
    pickerPosXPct = Math.max(0, Math.min(xPct, 100));
    pickerPosYPct = Math.max(0, Math.min(yPct, 100));
    renderPicker();
}

function renderPicker() {
    const rect = pickerBox.getBoundingClientRect();
    const xPx = Math.round((pickerPosXPct / 100) * rect.width);
    const yPx = Math.round((pickerPosYPct / 100) * rect.height);

    pickerHandle.style.transform = `translate(${xPx - 5}px, ${yPx - 5}px)`;

    const hue = (pickerPosXPct / 100) * 360;
    const lightness = 50 + (pickerPosYPct / 100) * 50;

    pickerHandle.style.borderColor = pickerPosYPct > 50 ? '#000' : '#fff';

    document.documentElement.style.setProperty('--hue', hue);
    document.documentElement.style.setProperty('--light', lightness + '%');
    updatePickerPreview(hue, lightness);
}

applyPickerPosition(0, 0);

new ResizeObserver(() => renderPicker()).observe(pickerBox);

function updatePickerFromEvent(e) {
    const rect = pickerBox.getBoundingClientRect();
    const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    const y = Math.max(0, Math.min(e.clientY - rect.top, rect.height));
    applyPickerPosition((x / rect.width) * 100, (y / rect.height) * 100);
}

let pickerIsDragging = false;
pickerBox.addEventListener('mousedown', (e) => {
    pickerIsDragging = true;
    pickerBox.style.cursor = 'none';
    updatePickerFromEvent(e);
});
window.addEventListener('mousemove', (e) => { if (pickerIsDragging) updatePickerFromEvent(e); });
window.addEventListener('mouseup', () => {
    if (pickerIsDragging) {
        pickerIsDragging = false;
        pickerBox.style.cursor = 'default';
    }
});

const PICKER_BOX_SLOW_PX = 0.1;
const PICKER_BOX_FAST_PX = 1;
const PICKER_BOX_TICK_MS = 10;
const PICKER_BOX_FAST_AFTER_MS = 900;

const PICKER_BOX_KEY_DELTAS = {
    ArrowLeft: { dx: -1, dy: 0 },
    ArrowRight: { dx: 1, dy: 0 },
    ArrowUp: { dx: 0, dy: -1 },
    ArrowDown: { dx: 0, dy: 1 },
};

const pressedPickerBoxKeys = new Set();
let pickerBoxMoveInterval = null;
let pickerBoxHoldStart = 0;

function pickerBoxMoveTick() {
    let dx = 0, dy = 0;
    pressedPickerBoxKeys.forEach((key) => {
        dx += PICKER_BOX_KEY_DELTAS[key].dx;
        dy += PICKER_BOX_KEY_DELTAS[key].dy;
    });
    if (dx === 0 && dy === 0) return;

    const elapsed = performance.now() - pickerBoxHoldStart;
    const speedPx = elapsed >= PICKER_BOX_FAST_AFTER_MS ? PICKER_BOX_FAST_PX : PICKER_BOX_SLOW_PX;

    const rect = pickerBox.getBoundingClientRect();
    const xPct = pickerPosXPct + (dx * speedPx / rect.width) * 100;
    const yPct = pickerPosYPct + (dy * speedPx / rect.height) * 100;
    applyPickerPosition(xPct, yPct);
}

function stopPickerBoxKey(key) {
    pressedPickerBoxKeys.delete(key);
    if (pressedPickerBoxKeys.size === 0 && pickerBoxMoveInterval) {
        clearInterval(pickerBoxMoveInterval);
        pickerBoxMoveInterval = null;
    }
}

pickerBox.addEventListener('keydown', (e) => {
    if (!(e.key in PICKER_BOX_KEY_DELTAS)) return;
    e.preventDefault();
    if (pressedPickerBoxKeys.has(e.key)) return;
    pressedPickerBoxKeys.add(e.key);
    if (!pickerBoxMoveInterval) {
        pickerBoxHoldStart = performance.now();
        pickerBoxMoveTick();
        pickerBoxMoveInterval = setInterval(pickerBoxMoveTick, PICKER_BOX_TICK_MS);
    }
});

pickerBox.addEventListener('keyup', (e) => {
    if (e.key in PICKER_BOX_KEY_DELTAS) stopPickerBoxKey(e.key);
});

pickerBox.addEventListener('blur', () => {
    pressedPickerBoxKeys.clear();
    if (pickerBoxMoveInterval) {
        clearInterval(pickerBoxMoveInterval);
        pickerBoxMoveInterval = null;
    }
});

pickerDarkness.addEventListener('input', (e) => {
    const darkValue = e.target.value / 100;
    document.documentElement.style.setProperty('--dark', darkValue);
    updatePickerPreview(pickerCurrentHue(), pickerCurrentLightness());
});

pickerTransparency.addEventListener('input', (e) => {
    const alphaValue = 1 - (e.target.value / 100);
    document.documentElement.style.setProperty('--alpha', alphaValue);
    updatePickerPreview(pickerCurrentHue(), pickerCurrentLightness());
});

function stepPickerRange(input, delta) {
    const min = parseFloat(input.min) || 0;
    const max = parseFloat(input.max) || 100;
    const step = parseFloat(input.step) || 1;
    const value = Math.max(min, Math.min(max, parseFloat(input.value) + delta * step));
    input.value = value;
    input.dispatchEvent(new Event('input', { bubbles: true }));
}

[pickerDarkness, pickerTransparency].forEach((input) => {
    input.addEventListener('keydown', (e) => {
        let delta = 0;
        if (e.key === 'ArrowDown' || e.key === 'ArrowRight') delta = 1;
        else if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') delta = -1;
        else return;
        e.preventDefault();
        stepPickerRange(input, delta);
    });
});

function syncPickerFromColor(rgba) {
    const { h, s, v } = rgbToHsv(rgba.r, rgba.g, rgba.b);
    const light = 100 - (s / 2);
    const dark = 1 - (v / 100);

    document.documentElement.style.setProperty('--hue', h);
    document.documentElement.style.setProperty('--sat', '100%');
    document.documentElement.style.setProperty('--light', `${light}%`);
    document.documentElement.style.setProperty('--dark', dark);
    document.documentElement.style.setProperty('--alpha', rgba.a);

    const xPct = (h / 360) * 100;
    const yPct = ((light - 50) / 50) * 100;
    applyPickerPosition(xPct, yPct);

    pickerDarkness.value = Math.round(dark * 100);
    pickerTransparency.value = Math.round((1 - rgba.a) * 100);

    const baseRgb = hslToRgb(h, 100, light);
    const darkRgb = {
        r: baseRgb.r * (1 - dark),
        g: baseRgb.g * (1 - dark),
        b: baseRgb.b * (1 - dark)
    };
    const baseCss = `rgb(${Math.round(baseRgb.r)}, ${Math.round(baseRgb.g)}, ${Math.round(baseRgb.b)})`;
    const darkCss = `rgb(${Math.round(darkRgb.r)}, ${Math.round(darkRgb.g)}, ${Math.round(darkRgb.b)})`;
    const finalCss = `rgba(${Math.round(darkRgb.r)}, ${Math.round(darkRgb.g)}, ${Math.round(darkRgb.b)}, ${rgba.a})`;

    pickerPreview.style.background = finalCss;
    pickerDarkness.style.background = `linear-gradient(${baseCss}, transparent)`;
    pickerTransparency.style.background = `linear-gradient(${darkCss}, transparent)`;

    if (!pickerBox.dataset.syncReleaseBound) {
        const release = () => {
            pickerPreview.style.background = '';
            pickerDarkness.style.background = '';
            pickerTransparency.style.background = '';
        };
        pickerBox.addEventListener('mousedown', release);
        pickerDarkness.addEventListener('input', release);
        pickerTransparency.addEventListener('input', release);
        pickerBox.dataset.syncReleaseBound = '1';
    }
}

function readPickerColor() {
    const computed = getComputedStyle(pickerPreview).backgroundColor;
    const canvas = document.createElement('canvas');
    canvas.width = 1;
    canvas.height = 1;
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, 1, 1);
    ctx.fillStyle = computed;
    ctx.fillRect(0, 0, 1, 1);
    const [r, g, b, a] = ctx.getImageData(0, 0, 1, 1).data;
    return { r, g, b, a: a / 255 };
}

function setupColorPanel() {
    const panel = document.getElementById('colorformat');
    if (!panel) return;
    const inputBox = panel.querySelector('.input-box');
    const dropdown = panel.querySelector('.dropdown');
    const content = panel.querySelector('.output-box .content');
    const copyButton = panel.querySelector('#copy.button');

    content.innerHTML = '';
    const swatch = document.createElement('span');
    swatch.className = 'color-swatch';
    swatch.tabIndex = 0;
    swatch.setAttribute('role', 'button');
    swatch.setAttribute('aria-label', 'Open color picker');
    const textSpan = document.createElement('span');
    textSpan.className = 'color-text';
    content.appendChild(swatch);
    content.appendChild(textSpan);

    function process() {
        const raw = inputBox.value;
        if (!raw.trim()) {
            swatch.style.visibility = 'hidden';
            textSpan.classList.remove('invalid');
            textSpan.textContent = '';
            return;
        }
        const rgba = parseColor(raw);
        if (!rgba) {
            swatch.style.visibility = 'hidden';
            textSpan.classList.add('invalid');
            textSpan.textContent = 'Invalid color';
            return;
        }
        swatch.style.visibility = 'visible';
        textSpan.classList.remove('invalid');
        const swatchAlpha = ALPHA_AWARE_FORMATS.has(dropdown.value) ? rgba.a : 1;
        swatch.style.setProperty('--swatch-color', `rgba(${rgba.r}, ${rgba.g}, ${rgba.b}, ${swatchAlpha})`);
        textSpan.textContent = formatColor(rgba, dropdown.value);
    }

    inputBox.addEventListener('input', process);
    dropdown.addEventListener('change', process);
    process();

    copyButton.addEventListener('click', () => {
        const raw = inputBox.value;
        if (!raw.trim()) { triggerWarn('copy-empty'); return; }
        const rgba = parseColor(raw);
        if (!rgba) { doCopyInvalid(); return; }
        doCopyValid(textSpan.textContent);
    });

    const pickerPanel = document.getElementById('colorpicker-panel');
    const pickerBack = document.getElementById('colorpicker-back');

    swatch.addEventListener('click', () => {
        const rgba = parseColor(inputBox.value);
        if (rgba) syncPickerFromColor(rgba);
        panel.classList.remove('active');
        pickerPanel?.classList.add('active');
        pickerBack?.focus({ focusVisible: true });
    });

    swatch.addEventListener('keydown', (e) => {
        if (e.key !== 'Enter' && e.key !== ' ') return;
        e.preventDefault();
        swatch.click();
    });

    pickerBack?.addEventListener('click', () => {
        const rgba = readPickerColor();
        if (rgba) {
            const opaque = Math.round(rgba.a * 255) >= 255;
            inputBox.value = formatColor(rgba, opaque ? 'option1' : 'option3');
            inputBox.dispatchEvent(new Event('input', { bubbles: true }));
        }
        pickerPanel?.classList.remove('active');
        panel.classList.add('active');
        swatch.focus({ focusVisible: true });
    });
}

function setupMediaPanel() {
    const panel = document.getElementById('mediafileconverter');
    if (!panel) return;
    const fileInput = panel.querySelector('.file-upload');
    const fileNameInput = panel.querySelector('.input-box');
    const dropdown = panel.querySelector('.dropdown');
    const optgroups = panel.querySelectorAll('.dropdown optgroup');
    const resetButton = panel.querySelector('#reset.button');
    const proceedButton = panel.querySelector('#proceed.button');
    const uploadZone = panel.querySelector('.file-upload-zone');
    const unattachButton = panel.querySelector('.unattach-file');
    const metadataCheckbox = panel.querySelector('#o-metadata input');
    const preserveCheckbox = panel.querySelector('#o-preserve input');
    const overwriteCheckbox = panel.querySelector('#o-overwrite input');
    const typeSvgs = {
        image: panel.querySelector('.image-svg'),
        video: panel.querySelector('.video-svg'),
        audio: panel.querySelector('.audio-svg')
    };

    const loadingPanel = document.getElementById('loading');
    const loadingBarFill = loadingPanel?.querySelector('.loading-bar-fill');
    const windowEl = document.querySelector('.window');
    const sidebarEl = document.querySelector('.sidebar');

    function setProgress(percent) {
        if (loadingBarFill) loadingBarFill.style.width = `${percent}%`;
    }

    function showLoading() {
        setProgress(0);
        void loadingBarFill?.offsetWidth;
        panel.classList.remove('active');
        loadingPanel?.classList.add('show');
        windowEl?.classList.add('hidden');
        sidebarEl?.classList.add('hidden');
    }

    function hideLoading() {
        loadingPanel?.classList.remove('show');
        panel.classList.add('active');
        windowEl?.classList.remove('hidden');
        sidebarEl?.classList.remove('hidden');
    }

    const isTauri = '__TAURI_INTERNALS__' in window;

    let originalFileName = '';
    let originalExtension = '';
    let originalFilePath = null;
    let userTypedBeforeUpload = false;

    fileNameInput.addEventListener('input', () => {
        if (panel.hasAttribute('toupload')) {
            userTypedBeforeUpload = fileNameInput.value.trim().length > 0;
        }
    });

    loadSettings().then((settings) => {
        metadataCheckbox.checked = !!settings.keep_metadata;
        preserveCheckbox.checked = !!settings.preserve_date;
        overwriteCheckbox.checked = !!settings.overwrite;
    });

    function persistCheckboxState() {
        saveSettings({
            keep_metadata: metadataCheckbox.checked,
            preserve_date: preserveCheckbox.checked,
            overwrite: overwriteCheckbox.checked
        });
    }

    [metadataCheckbox, preserveCheckbox, overwriteCheckbox].forEach((checkbox) => {
        checkbox.addEventListener('change', persistCheckboxState);
    });

    function getExtension(filename) {
        const idx = filename.lastIndexOf('.');
        return idx === -1 ? '' : filename.slice(idx + 1).toLowerCase();
    }

    function stripExtension(filename) {
        const idx = filename.lastIndexOf('.');
        return idx === -1 ? filename : filename.slice(0, idx);
    }

    function basename(path) {
        return path.split(/[\\/]/).pop();
    }

    function handleFile(name, path) {
        if (!name) return;

        const ext = getExtension(name);
        const matchingGroup = Array.from(optgroups).find(group =>
            Array.from(group.querySelectorAll('option')).some(opt => opt.value.toLowerCase() === ext)
        );

        if (!matchingGroup) {
            doFileInvalid();
            if (fileInput) fileInput.value = '';
            return;
        }

        optgroups.forEach(group => {
            const keepEnabled = group === matchingGroup
                || (matchingGroup.id === 'video' && group.id === 'audio');
            group.disabled = !keepEnabled;
        });

        const firstUsable = (group) => Array.from(group.querySelectorAll('option')).find(opt => !opt.disabled);
        const defaultOption = firstUsable(matchingGroup)
            || (matchingGroup.id === 'video' ? firstUsable(panel.querySelector('#audio')) : null);
        if (defaultOption) {
            dropdown.value = defaultOption.value;
            dropdown.__dropdownSync?.();
        }

        Object.entries(typeSvgs).forEach(([type, svg]) => {
            if (!svg) return;
            if (type === matchingGroup.id) svg.setAttribute('enabled', '');
            else svg.removeAttribute('enabled');
        });

        originalFileName = stripExtension(name);
        originalExtension = ext;
        originalFilePath = path;
        if (!userTypedBeforeUpload) {
            fileNameInput.value = originalFileName;
        }
        uploadZone.setAttribute('data-filename', name);

        panel.removeAttribute('toupload');
        if (document.activeElement === uploadZone) fileNameInput.focus();
        uploadZone.tabIndex = -1;
        if (unattachButton) unattachButton.tabIndex = 0;
    }

    function clearFile() {
        originalFileName = '';
        originalExtension = '';
        originalFilePath = null;
        userTypedBeforeUpload = false;

        if (fileInput) fileInput.value = '';
        fileNameInput.value = '';
        uploadZone.removeAttribute('data-filename');

        optgroups.forEach(group => { group.disabled = false; });
        Object.values(typeSvgs).forEach(svg => svg?.removeAttribute('enabled'));

        panel.setAttribute('toupload', '');
        uploadZone.tabIndex = 0;
        if (unattachButton) {
            if (document.activeElement === unattachButton) uploadZone.focus();
            unattachButton.tabIndex = -1;
        }
    }

    if (isTauri) {
        const { open } = window.__TAURI__.dialog;
        const { getCurrentWindow } = window.__TAURI__.window;

        uploadZone.addEventListener('click', (e) => {
            e.preventDefault();
            const supportedExtensions = [...new Set(
                Array.from(dropdown.querySelectorAll('option')).map(opt => opt.value)
            )];
            open({
                multiple: false,
                directory: false,
                title: 'Select a file to convert',
                filters: [{ name: 'Supported files', extensions: supportedExtensions }]
            }).then((path) => {
                if (!path) return;
                handleFile(basename(path), path);
            }).catch((err) => {
                console.error('File dialog failed:', err);
            });
        });

        getCurrentWindow().onDragDropEvent((event) => {
            if (event.payload.type !== 'drop') return;

            if (!panel.classList.contains('active')) return;

            const { x, y } = event.payload.position;
            const scale = window.devicePixelRatio || 1;
            const dropX = x / scale;
            const dropY = y / scale;
            const rect = uploadZone.getBoundingClientRect();
            const insideZone = dropX >= rect.left && dropX <= rect.right
                && dropY >= rect.top && dropY <= rect.bottom;
            if (!insideZone) return;

            const path = event.payload.paths?.[0];
            if (!path) return;
            handleFile(basename(path), path);
        });
    } else {
        fileInput.addEventListener('change', () => {
            const file = fileInput.files[0];
            if (file) handleFile(file.name, null);
        });

        uploadZone.addEventListener('dragover', (e) => e.preventDefault());
        uploadZone.addEventListener('drop', (e) => {
            e.preventDefault();
            const file = e.dataTransfer.files[0];
            if (file) handleFile(file.name, null);
        });
    }

    uploadZone.addEventListener('keydown', (e) => {
        if (e.key !== 'Enter' && e.key !== ' ') return;
        e.preventDefault();
        uploadZone.click();
    });

    unattachButton?.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        clearFile();
    }, { capture: true });

    unattachButton?.addEventListener('keydown', (e) => {
        if (e.key !== 'Enter' && e.key !== ' ') return;
        e.preventDefault();
        e.stopPropagation();
        unattachButton.click();
    });

    resetButton.addEventListener('click', () => {
        fileNameInput.value = originalFileName;
        [metadataCheckbox, preserveCheckbox, overwriteCheckbox].forEach(cb => {
            cb.checked = false;
        });
        persistCheckboxState();
    });

    proceedButton.addEventListener('click', async () => {
        if (!originalFilePath) {
            doFileNotAttached();
            return;
        }
        if (!fileNameInput.value.trim()) {
            doFileNameEmpty();
            return;
        }
        if (dropdown.value.toLowerCase() === originalExtension) {
            doMediaConvertInvalid();
            return;
        }
        if (!isTauri) {
            console.warn('Conversion needs the desktop app and a real file path.');
            return;
        }

        const mediaKind = typeSvgs.image?.hasAttribute('enabled') ? 'image'
            : typeSvgs.video?.hasAttribute('enabled') ? 'video'
            : typeSvgs.audio?.hasAttribute('enabled') ? 'audio'
            : null;

        const targetIsAudio = dropdown.selectedOptions[0]?.closest('optgroup')?.id === 'audio';
        const effectiveKind = mediaKind === 'video'
            ? (targetIsAudio ? 'audio' : 'video')
            : mediaKind;

        if (effectiveKind !== 'image' && effectiveKind !== 'audio' && effectiveKind !== 'video') {
            return;
        }

        const { invoke } = window.__TAURI__.core;
        const { listen } = window.__TAURI__.event;

        showLoading();
        const unlisten = await listen('conversion-progress', (event) => {
            setProgress(event.payload);
        });

        try {
            const outputPath = effectiveKind === 'image'
                ? await invoke('convert_image', {
                    sourcePath: originalFilePath,
                    outputName: fileNameInput.value || originalFileName,
                    targetExt: dropdown.value.toLowerCase(),
                    keepMetadata: metadataCheckbox.checked,
                    preserveDate: preserveCheckbox.checked,
                    overwrite: overwriteCheckbox.checked
                })
                : effectiveKind === 'video'
                ? await invoke('convert_video', {
                    sourcePath: originalFilePath,
                    outputName: fileNameInput.value || originalFileName,
                    targetExt: dropdown.value.toLowerCase(),
                    preserveDate: preserveCheckbox.checked,
                    overwrite: overwriteCheckbox.checked
                })
                : await invoke('convert_audio', {
                    sourcePath: originalFilePath,
                    outputName: fileNameInput.value || originalFileName,
                    targetExt: dropdown.value.toLowerCase(),
                    preserveDate: preserveCheckbox.checked,
                    overwrite: overwriteCheckbox.checked
                });
            console.log('Converted:', outputPath);
            setProgress(100);
            await new Promise((resolve) => setTimeout(resolve, 300));
            doFileSaved();
            clearFile();
        } catch (err) {
            console.error('Conversion failed:', err);
            doFileCorrupted(err);
            clearFile();
        } finally {
            unlisten();
            hideLoading();
        }
    });
}


function withNoTransition(callback) {
    document.body.setAttribute('theme-switch', '');
    callback();
    void document.body.offsetWidth;
    requestAnimationFrame(() => {
        document.body.removeAttribute('theme-switch');
    });
}

async function setupSettingsPanel() {
    const isTauri = '__TAURI_INTERNALS__' in window;
    const currentWindow = isTauri ? window.__TAURI__.window.getCurrentWindow() : null;

    const alwaysOnTopInput = document.getElementById('s-always-on-top');
    const rememberToolInput = document.getElementById('s-remember-tool');
    const closeOnFocusLossInput = document.getElementById('s-close-on-focus-loss');
    const hideWarningsInput = document.getElementById('s-hide-warnings');
    const appThemeInput = document.getElementById('s-app-theme');
    const highContrastInput = document.getElementById('s-high-contrast');

    const saved = await loadAppSettings();
    appSettings = {
        alwaysOnTop: !!saved.alwaysOnTop,
        rememberLastTool: !!saved.rememberLastTool,
        closeOnFocusLoss: !!saved.closeOnFocusLoss,
        hideWarnNotifications: !!saved.hideWarnNotifications,
        lastTool: saved.lastTool || 'fontstyler',
        appTheme: saved.appTheme || 'dark',
        highContrast: !!saved.highContrast
    };

    if (alwaysOnTopInput) alwaysOnTopInput.checked = appSettings.alwaysOnTop;
    if (rememberToolInput) rememberToolInput.checked = appSettings.rememberLastTool;
    if (closeOnFocusLossInput) closeOnFocusLossInput.checked = appSettings.closeOnFocusLoss;
    if (hideWarningsInput) hideWarningsInput.checked = appSettings.hideWarnNotifications;
    if (appThemeInput) {
        appThemeInput.value = appSettings.appTheme;
        appThemeInput.__dropdownSync?.();
    }
    if (highContrastInput) highContrastInput.checked = appSettings.highContrast;

    withNoTransition(() => {
        document.body.setAttribute('theme', appSettings.appTheme);
        document.body.toggleAttribute('contrast', appSettings.highContrast);
    });

    if (currentWindow && appSettings.alwaysOnTop) {
        currentWindow.setAlwaysOnTop(true).catch((err) => console.error('Could not set always-on-top:', err));
    }

    alwaysOnTopInput?.addEventListener('change', () => {
        appSettings.alwaysOnTop = alwaysOnTopInput.checked;
        saveAppSettings(appSettings);
        currentWindow?.setAlwaysOnTop(alwaysOnTopInput.checked)
            .catch((err) => console.error('Could not set always-on-top:', err));
    });

    rememberToolInput?.addEventListener('change', () => {
        appSettings.rememberLastTool = rememberToolInput.checked;
        saveAppSettings(appSettings);
    });

    closeOnFocusLossInput?.addEventListener('change', () => {
        appSettings.closeOnFocusLoss = closeOnFocusLossInput.checked;
        saveAppSettings(appSettings);
    });

    hideWarningsInput?.addEventListener('change', () => {
        appSettings.hideWarnNotifications = hideWarningsInput.checked;
        saveAppSettings(appSettings);
    });

    appThemeInput?.addEventListener('change', () => {
        appSettings.appTheme = appThemeInput.value;
        saveAppSettings(appSettings);
        withNoTransition(() => {
            document.body.setAttribute('theme', appThemeInput.value);
        });
    });

    highContrastInput?.addEventListener('change', () => {
        appSettings.highContrast = highContrastInput.checked;
        saveAppSettings(appSettings);
        withNoTransition(() => {
            document.body.toggleAttribute('contrast', highContrastInput.checked);
        });
    });

    currentWindow?.onFocusChanged(({ payload: focused }) => {
        if (!focused && appSettings.closeOnFocusLoss && Date.now() - appLaunchTime > 2000) {
            currentWindow.close();
        }
    });

    return (appSettings.rememberLastTool && document.getElementById(appSettings.lastTool))
        ? appSettings.lastTool
        : 'fontstyler';
}


function setupInputClearButtons() {
    document.querySelectorAll('.input-clear').forEach((button) => {
        const inputBox = button.previousElementSibling;
        if (!inputBox || !inputBox.classList.contains('input-box')) return;

        button.addEventListener('mousedown', (e) => e.preventDefault());

        button.addEventListener('click', () => {
            inputBox.value = '';
            inputBox.dispatchEvent(new Event('input', { bubbles: true }));
            inputBox.focus();
        });
    });
}

setupFontStyler();
setupColorPanel();
setupMediaPanel();
setupInputClearButtons();

let openDropdownState = null;

document.addEventListener('mousedown', (e) => {
    if (!openDropdownState) return;
    const { toggle, popup, close } = openDropdownState;
    if (e.button === 1) {
        e.preventDefault();
        close();
        return;
    }
    if (toggle.contains(e.target) || popup.contains(e.target)) return;
    close();
});

function findScrollableAncestor(el) {
    let node = el instanceof Element ? el : el?.parentElement;
    while (node && node !== document.body) {
        if (node.scrollHeight > node.clientHeight) {
            const overflowY = getComputedStyle(node).overflowY;
            if (overflowY === 'auto' || overflowY === 'scroll') return node;
        }
        node = node.parentElement;
    }
    return null;
}

document.addEventListener('wheel', (e) => {
    if (!openDropdownState) return;
    const { popup, close } = openDropdownState;
    if (popup.contains(e.target)) return;
    if (findScrollableAncestor(e.target)) {
        close();
    }
}, { passive: true });

window.addEventListener('blur', () => {
    if (!openDropdownState) return;
    openDropdownState.close();
});


const focusRing = document.createElement('div');
focusRing.className = 'focus-ring';
focusRing.setAttribute('popover', 'manual');
document.body.appendChild(focusRing);

function promoteFocusRing() {
    if (focusRing.matches(':popover-open')) focusRing.hidePopover();
    focusRing.showPopover();
}

function inflateRadius(radius, amount) {
    return radius.split(' ').map(part => {
        if (part.endsWith('%')) return part;
        const value = parseFloat(part);
        return isNaN(value) ? part : `${value + amount}px`;
    }).join(' ');
}

function getFocusVisualTarget(target) {
    return target.closest('.switch') || target;
}

function updateFocusRing(target, skipTransition = false) {
    target = getFocusVisualTarget(target);
    const rect = target.getBoundingClientRect();
    const radius = inflateRadius(getComputedStyle(target).borderRadius, 1);

    if (skipTransition) focusRing.classList.add('no-transition');
    focusRing.style.top = `${rect.top - 1}px`;
    focusRing.style.left = `${rect.left - 1}px`;
    focusRing.style.width = `${rect.width + 2}px`;
    focusRing.style.height = `${rect.height + 2}px`;
    focusRing.style.borderRadius = radius;
    focusRing.classList.add('visible');
    if (skipTransition) {
        void focusRing.offsetWidth;
        focusRing.classList.remove('no-transition');
    }
}

function hideFocusRing() {
    focusRing.classList.remove('visible');
    if (focusRing.matches(':popover-open')) focusRing.hidePopover();
}

let usingKeyboard = true;

document.addEventListener('pointerdown', () => {
    usingKeyboard = false;
    hideFocusRing();
    lastTrackedTarget = null;
    lastTrackedRect = null;
}, true);

document.addEventListener('keydown', () => {
    usingKeyboard = true;
}, true);

document.addEventListener('focusin', (e) => {
    const target = e.target;
    if (usingKeyboard && target.matches?.(':focus-visible')) {
        updateFocusRing(target);
        promoteFocusRing();
        lastTrackedTarget = getFocusVisualTarget(target);
        lastTrackedRect = lastTrackedTarget.getBoundingClientRect();
    } else {
        hideFocusRing();
        lastTrackedTarget = null;
        lastTrackedRect = null;
    }
});

document.addEventListener('focusout', () => {
    hideFocusRing();
    lastTrackedTarget = null;
    lastTrackedRect = null;
});

function rectsEqual(a, b) {
    return a.top === b.top && a.left === b.left &&
        a.width === b.width && a.height === b.height;
}

let lastTrackedTarget = null;
let lastTrackedRect = null;

function trackFocusRing() {
    if (!focusRing.classList.contains('visible') || !document.activeElement) return;

    const target = getFocusVisualTarget(document.activeElement);
    const rect = target.getBoundingClientRect();

    if (target !== lastTrackedTarget) {
        lastTrackedTarget = target;
        lastTrackedRect = rect;
        return;
    }

    if (rectsEqual(rect, lastTrackedRect)) return;

    updateFocusRing(target, true);
    lastTrackedRect = rect;
}
setInterval(trackFocusRing, (1000 / 60) / 10);


function initCustomDropdown(select) {
    if (select.dataset.customized) return;
    select.dataset.customized = '1';
    select.setAttribute('aria-hidden', 'true');
    select.tabIndex = -1;

    const toggle = document.createElement('button');
    toggle.type = 'button';
    toggle.className = 'dropdown-toggle';
    toggle.setAttribute('role', 'combobox');
    toggle.setAttribute('aria-haspopup', 'listbox');
    toggle.setAttribute('aria-expanded', 'false');

    const label = document.createElement('span');
    label.className = 'dropdown-toggle-label';
    toggle.appendChild(label);

    select.insertAdjacentElement('afterend', toggle);

    const popup = document.createElement('div');
    popup.className = 'dropdown-popup';
    popup.setAttribute('popover', 'manual');
    document.body.appendChild(popup);

    const scrollWrap = document.createElement('div');
    scrollWrap.className = 'dropdown-popup-scroll';
    scrollWrap.setAttribute('role', 'listbox');
    popup.appendChild(scrollWrap);

    const isSettingsScoped = !!select.closest('#settings');
    let isOpen = false;
    let closeCleanup = null;

    function clearCloseCleanup() {
        if (closeCleanup) {
            clearTimeout(closeCleanup.timer);
            scrollWrap.removeEventListener('transitionend', closeCleanup.handler);
            closeCleanup = null;
        }
    }

    function syncLabel() {
        label.textContent = select.options[select.selectedIndex]?.textContent || '';
    }
    syncLabel();
    select.__dropdownSync = syncLabel;

    function makeOptionEl(optionEl) {
        const item = document.createElement('div');
        item.className = 'dropdown-option';
        item.setAttribute('role', 'option');
        item.textContent = optionEl.textContent;
        item.dataset.value = optionEl.value;

        if (optionEl.disabled) {
            item.classList.add('disabled');
            item.setAttribute('aria-disabled', 'true');
        } else {
            item.tabIndex = -1;
            item.addEventListener('click', () => selectOption(optionEl));
        }
        if (optionEl.value === select.value) {
            item.classList.add('selected');
            item.setAttribute('aria-selected', 'true');
        }
        return item;
    }

    function buildOptions() {
        scrollWrap.innerHTML = '';
        Array.from(select.children).forEach((child) => {
            if (child.tagName === 'OPTION') {
                scrollWrap.appendChild(makeOptionEl(child));
            } else if (child.tagName === 'OPTGROUP') {
                if (child.disabled) return;
                const group = document.createElement('div');
                group.className = 'dropdown-optgroup';
                Array.from(child.children).forEach((opt) => {
                    if (opt.tagName === 'OPTION') group.appendChild(makeOptionEl(opt));
                });
                scrollWrap.appendChild(group);
            }
        });
    }

    function selectOption(optionEl) {
        select.value = optionEl.value;
        syncLabel();
        select.dispatchEvent(new Event('change', { bubbles: true }));
        closePopup();
        toggle.focus({ focusVisible: true });
    }

    function positionPopup() {
        const rect = toggle.getBoundingClientRect();
        popup.style.top = `${rect.bottom + 6}px`;
        popup.style.left = 'auto';
        popup.style.right = `${window.innerWidth - rect.right}px`;
        popup.style.minWidth = `${rect.width}px`;
        scrollWrap.style.maxHeight = isSettingsScoped ? '115px' : '174px';
    }

    function openPopup() {
        if (isOpen) return;
        clearCloseCleanup();
        isOpen = true;
        buildOptions();
        positionPopup();
        if (!popup.matches(':popover-open')) popup.showPopover();
        toggle.setAttribute('aria-expanded', 'true');
        toggle.classList.add('open');
        openDropdownState = { toggle, popup, select, close: closePopup };

        const maxHeight = parseFloat(scrollWrap.style.maxHeight) || scrollWrap.scrollHeight;
        const targetHeight = Math.min(scrollWrap.scrollHeight, maxHeight);
        scrollWrap.classList.toggle('is-scrollable', scrollWrap.scrollHeight > maxHeight);
        scrollWrap.style.transition = 'none';
        scrollWrap.style.height = '0px';
        scrollWrap.style.overflowY = 'hidden';
        void scrollWrap.offsetHeight;
        scrollWrap.style.transition = 'height .1s ease';
        scrollWrap.style.height = `${targetHeight}px`;

        const finishOpen = (e) => {
            if (e && (e.target !== scrollWrap || e.propertyName !== 'height')) return;
            scrollWrap.removeEventListener('transitionend', finishOpen);
            if (!isOpen) return;
            scrollWrap.style.height = '';
            scrollWrap.style.overflowY = '';
            scrollWrap.style.transition = '';
        };
        scrollWrap.addEventListener('transitionend', finishOpen);

        const target = popup.querySelector('.dropdown-option.selected:not(.disabled)')
            || popup.querySelector('.dropdown-option:not(.disabled)');
        if (target) {
            scrollWrap.scrollTop = Math.max(0, target.offsetTop - 4);
            target.focus({ focusVisible: true, preventScroll: true });
        }
    }

    function closePopup() {
        if (!isOpen) return;
        isOpen = false;
        toggle.setAttribute('aria-expanded', 'false');
        toggle.classList.remove('open');
        if (openDropdownState?.select === select) openDropdownState = null;

        clearCloseCleanup();

        const currentHeight = scrollWrap.getBoundingClientRect().height;
        scrollWrap.style.transition = 'none';
        scrollWrap.style.height = `${currentHeight}px`;
        scrollWrap.style.overflowY = 'hidden';
        void scrollWrap.offsetHeight;
        scrollWrap.style.transition = 'height .1s ease';
        scrollWrap.style.height = '0px';

        const finish = () => {
            if (isOpen) return;
            if (popup.matches(':popover-open')) popup.hidePopover();
            scrollWrap.style.height = '';
            scrollWrap.style.overflowY = '';
            scrollWrap.style.transition = '';
            closeCleanup = null;
        };
        const handler = (e) => {
            if (e.target !== scrollWrap || e.propertyName !== 'height') return;
            clearTimeout(timerId);
            finish();
        };
        scrollWrap.addEventListener('transitionend', handler);
        const timerId = setTimeout(finish, 150);
        closeCleanup = { timer: timerId, handler };
    }

    toggle.addEventListener('click', () => {
        if (isOpen) {
            closePopup();
            toggle.focus({ focusVisible: true });
        } else {
            openPopup();
        }
    });

    popup.addEventListener('keydown', (e) => {
        const items = Array.from(popup.querySelectorAll('.dropdown-option:not(.disabled)'));
        const currentIndex = items.indexOf(document.activeElement);

        if (e.key === 'Escape') {
            e.preventDefault();
            e.stopPropagation();
            closePopup();
            toggle.focus({ focusVisible: true });
            return;
        }
        if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
            e.preventDefault();
            if (!items.length) return;
            const dir = e.key === 'ArrowDown' ? 1 : -1;
            const nextIndex = currentIndex === -1 ? 0 : (currentIndex + dir + items.length) % items.length;
            items[nextIndex]?.focus({ focusVisible: true });
            return;
        }
        if (e.key === 'Tab') {
            e.preventDefault();
            if (!items.length) return;
            const dir = e.shiftKey ? -1 : 1;
            const nextIndex = currentIndex === -1 ? 0 : (currentIndex + dir + items.length) % items.length;
            items[nextIndex]?.focus({ focusVisible: true });
            return;
        }
        if (e.key === 'Enter' || e.key === ' ') {
            if (currentIndex === -1) return;
            e.preventDefault();
            const value = items[currentIndex].dataset.value;
            const optionEl = Array.from(select.querySelectorAll('option')).find(o => o.value === value);
            if (optionEl) selectOption(optionEl);
        }
    });

    popup.addEventListener('focusout', () => {
        requestAnimationFrame(() => {
            if (!isOpen) return;
            const active = document.activeElement;
            if (popup.contains(active) || active === toggle) return;
            closePopup();
        });
    });
}

document.querySelectorAll('select.dropdown').forEach(initCustomDropdown);

document.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter') return;
    const target = e.target;
    if (target.matches?.('input[type="checkbox"]')) {
        e.preventDefault();
        target.click();
    }
});


const sidebarButtons = document.querySelectorAll('.sidebar-button');
const panels = document.querySelectorAll('.window > div[id]');
const sidebarIndicator = document.getElementById('sidebar-indicator');

const INDICATOR_REST_HEIGHT = 16;
const INDICATOR_LEAD_HEIGHT = 10;
const INDICATOR_STRETCH_HEIGHT = 26;
const INDICATOR_PULL_MS = 150;
const INDICATOR_SETTLE_MS = 240;
const INDICATOR_PULL_EASE = 'cubic-bezier(0.16, 1, 0.3, 1)';
const INDICATOR_SETTLE_EASE = 'cubic-bezier(0.3, 1.4, 0.55, 1)';
let indicatorPositioned = false;
let indicatorSettleTimeout = null;
let indicatorHostButton = null;

function moveSidebarIndicator(button) {
    if (!sidebarIndicator || !button) return;

    clearTimeout(indicatorSettleTimeout);

    const buttonHeight = button.offsetHeight;
    const restTop = buttonHeight / 2 - INDICATOR_REST_HEIGHT / 2;
    const previousButton = indicatorHostButton;
    button.appendChild(sidebarIndicator);
    indicatorHostButton = button;

    if (!indicatorPositioned || document.body.classList.contains('skip-tab-anim')) {
        sidebarIndicator.style.transition = 'none';
        sidebarIndicator.style.top = `${restTop}px`;
        sidebarIndicator.style.height = `${INDICATOR_REST_HEIGHT}px`;
        sidebarIndicator.classList.add('show');
        void sidebarIndicator.offsetHeight;
        sidebarIndicator.style.transition = '';
        indicatorPositioned = true;
        return;
    }

    if (previousButton === button) return;

    const previousCenter = previousButton
        ? previousButton.offsetTop + previousButton.offsetHeight / 2
        : button.offsetTop + buttonHeight / 2;
    const newCenter = button.offsetTop + buttonHeight / 2;
    const movingDown = newCenter > previousCenter;
    const movingUp = newCenter < previousCenter;

    if (!movingDown && !movingUp) return;

    const leadTop = movingDown ? 0 : buttonHeight - INDICATOR_LEAD_HEIGHT;
    const stretchTop = movingDown ? 0 : buttonHeight - INDICATOR_STRETCH_HEIGHT;

    sidebarIndicator.style.transition = 'none';
    sidebarIndicator.style.top = `${leadTop}px`;
    sidebarIndicator.style.height = `${INDICATOR_LEAD_HEIGHT}px`;
    void sidebarIndicator.offsetHeight;

    sidebarIndicator.style.transition = `top ${INDICATOR_PULL_MS}ms ${INDICATOR_PULL_EASE}, height ${INDICATOR_PULL_MS}ms ${INDICATOR_PULL_EASE}`;
    requestAnimationFrame(() => {
        sidebarIndicator.style.top = `${stretchTop}px`;
        sidebarIndicator.style.height = `${INDICATOR_STRETCH_HEIGHT}px`;
    });

    indicatorSettleTimeout = setTimeout(() => {
        sidebarIndicator.style.transition = `top ${INDICATOR_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE}, height ${INDICATOR_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE}`;
        sidebarIndicator.style.top = `${restTop}px`;
        sidebarIndicator.style.height = `${INDICATOR_REST_HEIGHT}px`;
    }, INDICATOR_PULL_MS);
}

const TAB_TITLES = {
    fontstyler: 'Font Styler',
    colorformat: 'Color Format',
    mediafileconverter: 'Media File Converter',
    autoclicker: 'Auto Clicker',
    settings: 'Settings'
};

const titlebarTitle = document.getElementById('titlebar-title');

function updateWindowTitle(panelId) {
    const label = TAB_TITLES[panelId] || panelId;
    const fullTitle = `fTools | ${label}`;

    if (titlebarTitle) titlebarTitle.textContent = fullTitle;
    document.title = fullTitle;

    window.__TAURI__?.window?.getCurrentWindow?.()?.setTitle?.(fullTitle).catch(() => {});
}

function activatePanel(panelId) {
    const targetPanel = document.getElementById(panelId);
    if (!targetPanel) return;

    panels.forEach(panel => panel.classList.remove('active'));
    sidebarButtons.forEach(button => button.classList.remove('active'));

    targetPanel.classList.add('active');
    const activeButton = document.getElementById(`t-${panelId}`);
    activeButton?.classList.add('active');
    moveSidebarIndicator(activeButton);
    updateWindowTitle(panelId);
}

sidebarButtons.forEach(button => {
    button.addEventListener('click', () => {
        document.body.classList.remove('skip-tab-anim');
        const panelId = button.id.replace(/^t-/, '');
        activatePanel(panelId);
        if (appSettings.rememberLastTool && appSettings.lastTool !== panelId) {
            appSettings.lastTool = panelId;
            saveAppSettings(appSettings);
        }
    });
});

setupSettingsPanel().then((panelId) => {
    document.body.classList.add('skip-tab-anim');
    activatePanel(panelId);
});


(function setupNativeTitlebar() {
    try {
        const saved = JSON.parse(localStorage.getItem('ftools:app-settings') || '{}');
        document.body.setAttribute('theme', saved.appTheme || 'dark');
        if (saved.highContrast) document.body.setAttribute('contrast', '');
    } catch (err) {
    }

    const tauriWindow = window.__TAURI__?.window;
    const appWindow = tauriWindow?.getCurrentWindow?.();
    const minimizeButton = document.getElementById('titlebar-minimize');
    const closeButton = document.getElementById('titlebar-close');

    if (!appWindow) {
        console.error('Tauri window API is unavailable.');
        return;
    }

    minimizeButton?.addEventListener('click', async (event) => {
        event.preventDefault();
        event.stopImmediatePropagation();
        try {
            await appWindow.minimize();
        } catch (err) {
            console.error('Failed to minimize the window:', err);
        }
    });

    closeButton?.addEventListener('click', async (event) => {
        event.preventDefault();
        event.stopImmediatePropagation();
        try {
            await appWindow.close();
        } catch (err) {
            console.error('Failed to close the window:', err);
        }
    });

    appWindow.isFocused()
        .then((focused) => document.body.toggleAttribute('window-focused', focused))
        .catch((err) => console.error('Failed to read window focus state:', err));

    appWindow.onFocusChanged(({ payload: focused }) => {
        document.body.toggleAttribute('window-focused', focused);
    });
})();