const appLaunchTime = Date.now();
let currentPanelId = 'fontstyler';

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
const warnCountEl = document.getElementById('warn-count');
let warnRepeatId = null;
let warnRepeatCount = 1;
let pendingWarn = null;
let lastFocusedBeforeWarn = null;

warnBox.querySelectorAll('a[id]').forEach((el) => {
    el.dataset.defaultText = el.textContent;
    warnItems.set(el.id, el);
});

warnBox.addEventListener('animationend', (e) => {
    if (e.animationName === 'warn-out') {
        warnBox.classList.remove('show', 'hide');

        if (pendingWarn) {
            const next = pendingWarn;
            pendingWarn = null;
            displayWarn(next.id, next.message, false);
        } else {
            warnCountEl.classList.remove('show');
            warnRepeatId = null;
            if (warnBox.matches(':popover-open')) warnBox.hidePopover();
        }
    }
});

function dismissWarn() {
    if (!warnBox.classList.contains('show')) return;

    clearTimeout(warnBox.warnTimeout);
    warnBox.classList.remove('show');
    warnBox.classList.add('hide');
}

warnBox.addEventListener('click', () => {
    dismissWarn();
    restoreFocusFromWarn();
});

function restoreFocusFromWarn() {
    if (document.activeElement !== warnBox) return;

    const target = lastFocusedBeforeWarn;
    lastFocusedBeforeWarn = null;

    if (target && document.contains(target) && typeof target.focus === 'function') {
        target.focus({ focusVisible: true, preventScroll: true });
    } else {
        warnBox.blur();
    }
}

warnBox.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        restoreFocusFromWarn();
        return;
    }
    if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        dismissWarn();
        restoreFocusFromWarn();
    }
});

warnBox.addEventListener('focusout', () => {
    dismissWarn();
});

function promoteWarn() {
    if (warnBox.matches(':popover-open')) warnBox.hidePopover();
    warnBox.showPopover();
}

function triggerWarn(id, message) {
    if (appSettings.hideWarnNotifications) return;
    if (!warnItems.has(id)) return;

    const isShowing = warnBox.classList.contains('show');
    const isRepeat = isShowing && warnRepeatId === id;

    if (isRepeat) {
        displayWarn(id, message, true);
        return;
    }

    if (isShowing) {
        pendingWarn = { id, message };
        dismissWarn();
        return;
    }

    displayWarn(id, message, false);
}

function displayWarn(id, message, isRepeat) {
    const activeEl = warnItems.get(id);
    if (!activeEl) return;

    const shouldFocusWarn = usingKeyboard && focusRing.classList.contains('visible') && document.activeElement !== warnBox;
    if (shouldFocusWarn) lastFocusedBeforeWarn = document.activeElement;

    let text = activeEl.dataset.defaultText;
    if (message) {
        const custom = String(message);
        text = custom.length > 140 ? `${custom.slice(0, 140)}…` : custom;
    }
    activeEl.textContent = text;

    warnItems.forEach((el) => el.classList.toggle('active', el === activeEl));

    warnRepeatId = id;
    warnRepeatCount = isRepeat ? warnRepeatCount + 1 : 1;

    if (warnRepeatCount > 1) {
        warnCountEl.textContent = warnRepeatCount > 9 ? '+9' : String(warnRepeatCount);
        warnCountEl.classList.add('show');
    } else {
        warnCountEl.classList.remove('show');
    }

    if (!isRepeat) {
        promoteWarn();
        warnBox.classList.remove('show', 'hide');
        void warnBox.offsetWidth;
        warnBox.classList.add('show');
    }

    if (shouldFocusWarn) warnBox.focus({ focusVisible: true, preventScroll: true });

    clearTimeout(warnBox.warnTimeout);
    warnBox.warnTimeout = setTimeout(() => {
        dismissWarn();
        restoreFocusFromWarn();
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

function doImageLoadFailed() {
    triggerWarn('image-load-failed');
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
    highContrast: false,
    hideToolInTitlebar: false,
    backgroundImagePath: null,
    backgroundMediaType: 'image'
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
    updatePickerSliderTooltips();
}

function updatePickerSliderTooltips() {
    pickerDarkness.setAttribute('tooltip', `Darkness: ${Math.round(pickerDarkness.value)}%`);
    pickerTransparency.setAttribute('tooltip', `Transparency: ${Math.round(pickerTransparency.value)}%`);
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
    updatePickerSliderTooltips();

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
    swatch.setAttribute('tooltip', 'Open color picker');
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

    const loadingPanel = document.getElementById('loading-screen');
    const loadingBarFill = loadingPanel?.querySelector('.loading-bar-fill');
    const windowEl = document.querySelector('.window');
    const sidebarEl = document.querySelector('.sidebar');

    let titleAnimFrame = null;
    let titleDisplayedPercent = 0;

    function animateTitleTo(target) {
        cancelAnimationFrame(titleAnimFrame);
        const start = titleDisplayedPercent;
        const startTime = performance.now();
        const duration = 400;

        function tick(now) {
            const t = Math.min((now - startTime) / duration, 1);
            const eased = 1 - Math.pow(1 - t, 3);
            titleDisplayedPercent = start + (target - start) * eased;
            setConversionProgressTitle(titleDisplayedPercent);
            if (t < 1) titleAnimFrame = requestAnimationFrame(tick);
        }
        titleAnimFrame = requestAnimationFrame(tick);
    }

    function setProgress(percent) {
        if (loadingBarFill) loadingBarFill.style.width = `${percent}%`;
        animateTitleTo(percent);
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
        cancelAnimationFrame(titleAnimFrame);
        loadingPanel?.classList.remove('show');
        panel.classList.add('active');
        windowEl?.classList.remove('hidden');
        sidebarEl?.classList.remove('hidden');
        updateWindowTitle(currentPanelId);
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

    const extraVideoTargets = { gif: ['mp4', 'mov', 'mkv', 'webm'] };

    function applySourceRestrictions(ext) {
        panel.querySelectorAll('#video option[data-source-locked]').forEach(opt => {
            opt.disabled = false;
            delete opt.dataset.sourceLocked;
        });
        const allowed = extraVideoTargets[ext];
        if (!allowed) return;
        panel.querySelectorAll('#video option').forEach(opt => {
            if (!opt.disabled && !allowed.includes(opt.value)) {
                opt.disabled = true;
                opt.dataset.sourceLocked = '';
            }
        });
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

        applySourceRestrictions(ext);
        optgroups.forEach(group => {
            const keepEnabled = group === matchingGroup
                || (matchingGroup.id === 'video' && group.id === 'audio')
                || (group.id === 'video' && ext in extraVideoTargets);
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

        applySourceRestrictions('');
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
            open({
                multiple: false,
                directory: false,
                title: 'Select a file to convert'
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

        const targetGroup = dropdown.selectedOptions[0]?.closest('optgroup')?.id;
        const effectiveKind = mediaKind === 'video' && targetGroup === 'audio' ? 'audio'
            : mediaKind === 'image' && targetGroup === 'video' ? 'video'
            : mediaKind;

        if (effectiveKind !== 'image' && effectiveKind !== 'audio' && effectiveKind !== 'video') {
            return;
        }

        const { invoke } = window.__TAURI__.core;
        const { listen } = window.__TAURI__.event;
        const outputName = fileNameInput.value || originalFileName;
        const targetExt = dropdown.value.toLowerCase();

        const existingFolder = await invoke('existing_output_folder', {
            sourcePath: originalFilePath,
            outputName,
            targetExt
        }).catch((err) => {
            console.error('Could not check for an existing output file:', err);
            return null;
        });
        if (existingFolder !== null) {
            const choice = await showDialog({
                title: 'Replace existing file?',
                message: `“${outputName}.${targetExt}” already exists in “${existingFolder}”. Proceeding will permanently delete the old file.`,
                buttons: [
                    { label: 'Replace', value: 'replace', featured: true },
                    { label: 'Cancel', value: 'cancel', focus: true }
                ]
            });
            if (choice !== 'replace') return;
        }

        showLoading();
        const unlisten = await listen('conversion-progress', (event) => {
            setProgress(event.payload);
        });

        try {
            const outputPath = effectiveKind === 'image'
                ? await invoke('convert_image', {
                    sourcePath: originalFilePath,
                    outputName,
                    targetExt,
                    keepMetadata: metadataCheckbox.checked,
                    preserveDate: preserveCheckbox.checked,
                    overwrite: overwriteCheckbox.checked
                })
                : effectiveKind === 'video'
                ? await invoke('convert_video', {
                    sourcePath: originalFilePath,
                    outputName,
                    targetExt,
                    preserveDate: preserveCheckbox.checked,
                    overwrite: overwriteCheckbox.checked
                })
                : await invoke('convert_audio', {
                    sourcePath: originalFilePath,
                    outputName,
                    targetExt,
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


const QR_SETTINGS_KEY = 'ftools:qrcode-settings';

function loadQrSettings() {
    try {
        const raw = localStorage.getItem(QR_SETTINGS_KEY);
        return raw ? JSON.parse(raw) : {};
    } catch (err) {
        console.error('Could not load QR code settings:', err);
        return {};
    }
}

function saveQrSettings(settings) {
    try {
        localStorage.setItem(QR_SETTINGS_KEY, JSON.stringify(settings));
    } catch (err) {
        console.error('Could not save QR code settings:', err);
    }
}

function setupQrConverter() {
    const panel = document.getElementById('qrcodeconverter');
    if (!panel) return;
    if (typeof qrcode !== 'function') {
        console.error('QR library not loaded; make sure qrcode/generator.js is included before app.js.');
        return;
    }

    const textInput = document.getElementById('qr-text');
    const eclSelect = document.getElementById('qr-ecl');
    const eclRow = document.getElementById('qr-ecl-row');
    const eclStops = Array.from(document.querySelectorAll('#qr-ecl-stops .qr-ecl-stop'));
    const eclTrack = document.getElementById('qr-ecl-track');
    const eclThumbVisual = document.getElementById('qr-ecl-thumb');
    const eclTrackFill = document.getElementById('qr-ecl-fill');
    const previewWrap = document.getElementById('qr-preview-wrap');
    const previewCanvas = document.getElementById('qr-preview-canvas');
    const saveImageBtn = document.getElementById('qr-save-image');
    const copyImageBtn = document.getElementById('qr-copy-image');

    if (!textInput || !eclSelect || !previewCanvas) return;

    const isTauri = '__TAURI_INTERNALS__' in window;
    const previewCtx = previewCanvas.getContext('2d');

    let lastQrDataUrl = null;

    function setPreviewEmpty(isEmpty) {
        previewWrap?.classList.toggle('empty', isEmpty);
    }

    function drawQr(text, ecl) {
        const qr = qrcode(0, ecl);
        qr.addData(text);
        qr.make();

        const count = qr.getModuleCount();
        const cellSize = Math.max(1, Math.floor(120 / count));
        const size = cellSize * count;

        previewCanvas.width = size;
        previewCanvas.height = size;
        previewCtx.fillStyle = '#fff';
        previewCtx.fillRect(0, 0, size, size);
        previewCtx.fillStyle = '#000';
        for (let row = 0; row < count; row += 1) {
            for (let col = 0; col < count; col += 1) {
                if (qr.isDark(row, col)) {
                    previewCtx.fillRect(col * cellSize, row * cellSize, cellSize, cellSize);
                }
            }
        }

        lastQrDataUrl = previewCanvas.toDataURL('image/png');
    }

    function regenerate() {
        const text = textInput.value;
        if (!text.trim()) {
            setPreviewEmpty(true);
            lastQrDataUrl = null;
            return;
        }

        try {
            drawQr(text, eclSelect.value);
            setPreviewEmpty(false);
        } catch (err) {
            console.error('Could not generate QR code:', err);
            setPreviewEmpty(true);
            lastQrDataUrl = null;
        }
    }

    let regenTimer = null;
    textInput.addEventListener('input', () => {
        clearTimeout(regenTimer);
        regenTimer = setTimeout(regenerate, 120);
    });

    eclSelect.addEventListener('change', () => {
        saveQrSettings({ ecl: eclSelect.value });
        regenerate();
    });

    const eclValues = ['L', 'M', 'Q', 'H'];
    const eclTrackWrap = eclTrack?.parentElement || null;

    let eclPointerId = null;

    function setEclThumbLeft(percent) {
        if (!eclTrackWrap || !eclThumbVisual) return;
        const trackWidth = eclTrackWrap.clientWidth;
        const thumbWidth = eclThumbVisual.offsetWidth || 14;
        const clamped = Math.min(Math.max(percent, 0), 1);
        const left = clamped * (trackWidth - thumbWidth);
        eclThumbVisual.style.left = `${left}px`;
        if (eclTrackFill) {
            eclTrackFill.style.width = `${left + thumbWidth / 2}px`;
        }
    }

    function updateEclThumbVisual() {
        if (!eclTrack) return;
        const min = Number(eclTrack.min) || 0;
        const max = Number(eclTrack.max) || 1;
        setEclThumbLeft((Number(eclTrack.value) - min) / (max - min));
    }

    if (eclTrackWrap && typeof ResizeObserver !== 'undefined') {
        const eclResizeObserver = new ResizeObserver(() => updateEclThumbVisual());
        eclResizeObserver.observe(eclTrackWrap);
    }

    function eclIndexFromClientX(clientX) {
        if (!eclTrackWrap) return 0;
        const rect = eclTrackWrap.getBoundingClientRect();
        const thumbWidth = eclThumbVisual?.offsetWidth || 14;
        const usable = rect.width - thumbWidth;
        const x = clientX - rect.left - thumbWidth / 2;
        const percent = usable > 0 ? Math.min(Math.max(x / usable, 0), 1) : 0;
        return Math.round(percent * (eclValues.length - 1));
    }

    function setEclThumb(value) {
        const index = eclValues.indexOf(value);
        if (index === -1 || !eclTrack) return;
        eclTrack.value = String(index);
        updateEclThumbVisual();
    }

    function setEclSelected(value) {
        eclStops.forEach((btn) => btn.classList.toggle('selected', btn.dataset.value === value));
        setEclThumb(value);
        const level = eclSelect.querySelector(`option[value="${value}"]`)?.textContent;
        if (level) eclRow?.setAttribute('tooltip', `Error correction: ${level}`);
    }

    function setEcl(value) {
        if (!eclValues.includes(value)) return;
        setEclSelected(value);
        if (eclSelect.value !== value) {
            eclSelect.value = value;
            eclSelect.dispatchEvent(new Event('change'));
        }
    }

    eclStops.forEach((btn) => {
        btn.addEventListener('click', () => setEcl(btn.dataset.value));
    });

    eclTrack?.addEventListener('input', () => {
        setEcl(eclValues[Number(eclTrack.value)]);
    });

    eclTrackWrap?.addEventListener('pointerdown', (e) => {
        eclPointerId = e.pointerId;
        eclTrackWrap.setPointerCapture(e.pointerId);
        setEcl(eclValues[eclIndexFromClientX(e.clientX)]);
    });

    eclTrackWrap?.addEventListener('pointermove', (e) => {
        if (eclPointerId === null || e.pointerId !== eclPointerId) return;
        setEcl(eclValues[eclIndexFromClientX(e.clientX)]);
    });

    function endEclDrag() {
        eclPointerId = null;
    }

    eclTrackWrap?.addEventListener('pointerup', endEclDrag);
    eclTrackWrap?.addEventListener('pointercancel', endEclDrag);

    setEclSelected(eclSelect.value);

    copyImageBtn?.addEventListener('click', async () => {
        if (!lastQrDataUrl) {
            triggerWarn('qr-no-code');
            return;
        }
        try {
            const blob = await (await fetch(lastQrDataUrl)).blob();
            await navigator.clipboard.write([new ClipboardItem({ [blob.type]: blob })]);
            triggerWarn('copy-valid');
        } catch (err) {
            console.error('Could not copy QR code image:', err);
            doCopyInvalid();
        }
    });

    saveImageBtn?.addEventListener('click', async () => {
        if (!lastQrDataUrl) {
            triggerWarn('qr-no-code');
            return;
        }
        try {
            const blob = await (await fetch(lastQrDataUrl)).blob();
            if (isTauri && window.__TAURI__?.dialog?.save && window.__TAURI__?.fs?.writeFile) {
                const path = await window.__TAURI__.dialog.save({
                    title: 'Save QR code',
                    defaultPath: 'qrcode.png',
                    filters: [{ name: 'PNG image', extensions: ['png'] }]
                });
                if (!path) return;
                const bytes = new Uint8Array(await blob.arrayBuffer());
                await window.__TAURI__.fs.writeFile(path, bytes);
            } else {
                const url = URL.createObjectURL(blob);
                const link = document.createElement('a');
                link.href = url;
                link.download = 'qrcode.png';
                document.body.appendChild(link);
                link.click();
                link.remove();
                URL.revokeObjectURL(url);
            }
            doFileSaved();
        } catch (err) {
            console.error('Could not save QR code image:', err);
            doFileCorrupted(err);
        }
    });

    const saved = loadQrSettings();
    if (saved.ecl) eclSelect.value = saved.ecl;
    setEclSelected(eclSelect.value);
    updateEclThumbVisual();
    setPreviewEmpty(true);
}


function withNoTransition(callback) {
    document.body.setAttribute('theme-switch', '');
    callback();
    void document.body.offsetWidth;
    requestAnimationFrame(() => {
        document.body.removeAttribute('theme-switch');
    });
}

function getRowDefault(input) {
    const row = input?.closest('.settings-row');
    const raw = row?.getAttribute('default');
    if (raw == null) return undefined;
    if (input.type === 'checkbox') return raw.toLowerCase() === 'enabled';
    return raw.toLowerCase();
}

function averageLumaFromSource(drawable, size = 32) {
    const canvas = document.createElement('canvas');
    canvas.width = size;
    canvas.height = size;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(drawable, 0, 0, size, size);
    const { data } = ctx.getImageData(0, 0, size, size);
    let total = 0;
    let count = 0;
    for (let i = 0; i < data.length; i += 4) {
        const alpha = data[i + 3];
        if (alpha === 0) continue;
        total += 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
        count++;
    }
    return count ? total / count : 128;
}

function getImageBrightness(src) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.crossOrigin = 'anonymous';
        img.onload = () => {
            try {
                resolve(averageLumaFromSource(img));
            } catch (err) {
                reject(err);
            }
        };
        img.onerror = reject;
        img.src = src;
    });
}

function getVideoBrightness(videoEl) {
    return new Promise((resolve, reject) => {
        if (!videoEl) { reject(new Error('No video element')); return; }
        const sample = () => {
            try {
                resolve(averageLumaFromSource(videoEl));
            } catch (err) {
                reject(err);
            }
        };
        if (videoEl.readyState >= 2) {
            sample();
        } else {
            videoEl.addEventListener('loadeddata', sample, { once: true });
            videoEl.addEventListener('error', reject, { once: true });
        }
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
    const hideToolInTitlebarInput = document.getElementById('s-hide-tool-in-titlebar');
    const backgroundImageButton = document.getElementById('s-background-image');
    const backgroundRemoveButton = document.getElementById('s-background-remove');
    const backgroundRow = document.getElementById('s-background-row');
    const backgroundFileInput = document.getElementById('s-background-file-input');
    const appBackgroundEl = document.getElementById('app-background');
    const appBackgroundVideoEl = document.getElementById('app-background-video');

    const BACKGROUND_IMAGE_EXTENSIONS = ['png', 'jpg', 'jpeg', 'bmp', 'gif', 'webp'];
    const BACKGROUND_VIDEO_EXTENSIONS = ['mp4', 'webm', 'mov', 'm4v', 'ogv'];
    const BG_BRIGHTNESS_SAMPLE_MS = 400;

    function getExtension(nameOrPath) {
        return nameOrPath?.split('.').pop()?.toLowerCase() || '';
    }

    function isVideoPath(path) {
        return BACKGROUND_VIDEO_EXTENSIONS.includes(getExtension(path));
    }

    function isImagePath(path) {
        return BACKGROUND_IMAGE_EXTENSIONS.includes(getExtension(path));
    }

    function isGifSource(src) {
        return src.startsWith('data:') ? src.startsWith('data:image/gif') : getExtension(src) === 'gif';
    }

    // Hard lock: the background video must never show native controls and must
    // never be reachable by click, double-click, right-click or keyboard.
    // This is intentionally redundant with the CSS pointer-events:none and the
    // missing "controls" attribute, since this element is purely decorative.
    function lockDownBackgroundVideo() {
        if (!appBackgroundVideoEl) return;
        appBackgroundVideoEl.controls = false;
        appBackgroundVideoEl.removeAttribute('controls');
        appBackgroundVideoEl.crossOrigin = 'anonymous';
        appBackgroundVideoEl.muted = true;
        appBackgroundVideoEl.defaultMuted = true;
        appBackgroundVideoEl.volume = 0;
        appBackgroundVideoEl.disablePictureInPicture = true;
        appBackgroundVideoEl.tabIndex = -1;
    }
    lockDownBackgroundVideo();
    appBackgroundVideoEl?.addEventListener('contextmenu', (e) => e.preventDefault());
    appBackgroundVideoEl?.addEventListener('keydown', (e) => e.preventDefault());
    appBackgroundVideoEl?.addEventListener('dblclick', (e) => e.preventDefault());
    appBackgroundVideoEl?.addEventListener('click', (e) => e.preventDefault());

    // Keeps bg-brightness live for content that changes over time (animated
    // GIFs, playing video) instead of sampling once and going stale.
    let bgBrightnessTimer = null;
    let bgSampleImg = null;

    // Remembers what the background currently is so it can resume seamlessly
    // when the theme switches back to "transparent" without recomputing the
    // Tauri file path or re-reading the file input.
    let currentBgSrc = null;
    let currentBgIsVideo = false;

    // The background layer (image/video/GIF luma sampling) is only ever
    // visible in the "transparent" theme (see .app-background in styles.css).
    // In every other theme it is fully hidden, so there is no reason to keep
    // decoding video frames or re-sampling brightness for it.
    function isTransparentTheme() {
        return appSettings.appTheme === 'transparent';
    }

    function updateBgBrightness(brightness) {
        withNoTransition(() => {
            document.body.setAttribute('bg-brightness', brightness > 140 ? 'light' : 'dark');
        });
    }

    function clearBgBrightness() {
        withNoTransition(() => {
            document.body.removeAttribute('bg-brightness');
        });
    }

    function stopBgBrightnessLoop() {
        if (bgBrightnessTimer) {
            clearInterval(bgBrightnessTimer);
            bgBrightnessTimer = null;
        }
        if (bgSampleImg) {
            bgSampleImg.src = '';
        }
        bgSampleImg = null;
    }

    // Performance mode: called whenever the theme leaves "transparent". The
    // background layer is already hidden by CSS at that point, so this stops
    // the invisible work behind it: the brightness polling loop, the hidden
    // GIF sampler image, and (unlike an off-screen GIF) the background video,
    // which keeps decoding frames even while hidden unless paused explicitly.
    function pauseBackgroundForPerformance() {
        stopBgBrightnessLoop();
        clearBgBrightness();
        appBackgroundVideoEl?.pause();
    }

    // Called whenever the theme returns to "transparent". Resumes exactly
    // what was showing before, from currentBgSrc, so the switch is seamless.
    function resumeBackgroundForPerformance() {
        if (!currentBgSrc) return;
        if (currentBgIsVideo && appBackgroundVideoEl) {
            appBackgroundVideoEl.play().catch((err) => console.error('Background video could not resume:', err));
            startVideoBrightnessTracking(appBackgroundVideoEl);
        } else if (!currentBgIsVideo) {
            startImageBrightnessTracking(currentBgSrc);
        }
    }

    function startImageBrightnessTracking(src) {
        getImageBrightness(src)
            .then(updateBgBrightness)
            .catch((err) => {
                console.error('Could not read background image brightness:', err);
                clearBgBrightness();
            });

        if (!isGifSource(src)) return;

        bgSampleImg = new Image();
        bgSampleImg.crossOrigin = 'anonymous';
        bgSampleImg.src = src;
        bgBrightnessTimer = setInterval(() => {
            if (!bgSampleImg || !bgSampleImg.complete || !bgSampleImg.naturalWidth) return;
            try {
                updateBgBrightness(averageLumaFromSource(bgSampleImg));
            } catch (err) {
                // Ignore transient decode errors between GIF frames.
            }
        }, BG_BRIGHTNESS_SAMPLE_MS);
    }

    function startVideoBrightnessTracking(videoEl) {
        getVideoBrightness(videoEl)
            .then(updateBgBrightness)
            .catch((err) => {
                console.error('Could not read background video brightness:', err);
                clearBgBrightness();
            });

        bgBrightnessTimer = setInterval(() => {
            if (videoEl.paused || videoEl.readyState < 2) return;
            try {
                updateBgBrightness(averageLumaFromSource(videoEl));
            } catch (err) {
                // Ignore transient decode errors mid-seek.
            }
        }, BG_BRIGHTNESS_SAMPLE_MS);
    }

    function applyBackgroundMedia() {
        if (!appBackgroundEl) return;
        stopBgBrightnessLoop();
        const path = appSettings.backgroundImagePath;
        const isVideo = appSettings.backgroundMediaType === 'video';

        if (!path) {
            currentBgSrc = null;
            currentBgIsVideo = false;
            appBackgroundEl.style.backgroundImage = '';
            if (appBackgroundVideoEl) {
                appBackgroundVideoEl.pause();
                appBackgroundVideoEl.removeAttribute('src');
                appBackgroundVideoEl.load();
            }
            document.body.removeAttribute('background-active');
            document.body.removeAttribute('background-media');
            clearBgBrightness();
            backgroundRow?.removeAttribute('has-background');
            if (backgroundRemoveButton) backgroundRemoveButton.tabIndex = -1;
            return;
        }

        const src = isTauri
            ? window.__TAURI__.core.convertFileSrc(path)
            : path;
        currentBgSrc = src;
        currentBgIsVideo = isVideo;
        document.body.setAttribute('background-active', '');
        backgroundRow?.setAttribute('has-background', '');
        if (backgroundRemoveButton) backgroundRemoveButton.tabIndex = 0;

        if (isVideo && appBackgroundVideoEl) {
            appBackgroundEl.style.backgroundImage = '';
            document.body.setAttribute('background-media', 'video');
            lockDownBackgroundVideo();
            appBackgroundVideoEl.src = src;
            appBackgroundVideoEl.load();
            if (isTransparentTheme()) {
                appBackgroundVideoEl.play().catch((err) => console.error('Background video could not autoplay:', err));
                startVideoBrightnessTracking(appBackgroundVideoEl);
            } else {
                // load() can otherwise re-trigger the <video autoplay> HTML
                // attribute; keep it frozen since it isn't visible here.
                appBackgroundVideoEl.pause();
            }
        } else {
            document.body.setAttribute('background-media', 'image');
            if (appBackgroundVideoEl) {
                appBackgroundVideoEl.pause();
                appBackgroundVideoEl.removeAttribute('src');
                appBackgroundVideoEl.load();
            }
            appBackgroundEl.style.backgroundImage = `url("${src}")`;
            if (isTransparentTheme()) {
                startImageBrightnessTracking(src);
            }
        }
    }

    const saved = await loadAppSettings();
    appSettings = {
        alwaysOnTop: saved.alwaysOnTop ?? getRowDefault(alwaysOnTopInput) ?? false,
        rememberLastTool: saved.rememberLastTool ?? getRowDefault(rememberToolInput) ?? false,
        closeOnFocusLoss: saved.closeOnFocusLoss ?? getRowDefault(closeOnFocusLossInput) ?? false,
        hideWarnNotifications: saved.hideWarnNotifications ?? getRowDefault(hideWarningsInput) ?? false,
        lastTool: saved.lastTool || 'fontstyler',
        appTheme: saved.appTheme || getRowDefault(appThemeInput) || 'dark',
        highContrast: saved.highContrast ?? getRowDefault(highContrastInput) ?? false,
        hideToolInTitlebar: saved.hideToolInTitlebar ?? getRowDefault(hideToolInTitlebarInput) ?? false,
        backgroundImagePath: saved.backgroundImagePath ?? null,
        backgroundMediaType: saved.backgroundMediaType ?? 'image'
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
    if (hideToolInTitlebarInput) hideToolInTitlebarInput.checked = appSettings.hideToolInTitlebar;

    withNoTransition(() => {
        document.body.setAttribute('theme', appSettings.appTheme);
        document.body.toggleAttribute('contrast', appSettings.highContrast);
    });

    applyBackgroundMedia();

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
        const previousTheme = appSettings.appTheme;
        appSettings.appTheme = appThemeInput.value;
        saveAppSettings(appSettings);
        withNoTransition(() => {
            document.body.setAttribute('theme', appThemeInput.value);
        });
        if (previousTheme === 'transparent' && appThemeInput.value !== 'transparent') {
            pauseBackgroundForPerformance();
        } else if (previousTheme !== 'transparent' && appThemeInput.value === 'transparent') {
            resumeBackgroundForPerformance();
        }
    });

    highContrastInput?.addEventListener('change', () => {
        appSettings.highContrast = highContrastInput.checked;
        saveAppSettings(appSettings);
        withNoTransition(() => {
            document.body.toggleAttribute('contrast', highContrastInput.checked);
        });
    });

    hideToolInTitlebarInput?.addEventListener('change', () => {
        appSettings.hideToolInTitlebar = hideToolInTitlebarInput.checked;
        saveAppSettings(appSettings);
        updateWindowTitle(currentPanelId);
    });

    backgroundImageButton?.addEventListener('click', () => {
        if (isTauri) {
            const { open } = window.__TAURI__.dialog;
            open({
                multiple: false,
                directory: false,
                title: 'Select a background image or video'
            }).then((path) => {
                if (!path) return;
                if (isVideoPath(path)) {
                    appSettings.backgroundMediaType = 'video';
                } else if (isImagePath(path)) {
                    appSettings.backgroundMediaType = 'image';
                } else {
                    doFileInvalid();
                    return;
                }
                appSettings.backgroundImagePath = path;
                saveAppSettings(appSettings);
                applyBackgroundMedia();
            }).catch((err) => console.error('Background media dialog failed:', err));
        } else {
            backgroundFileInput?.click();
        }
    });

    backgroundFileInput?.addEventListener('change', () => {
        const file = backgroundFileInput.files?.[0];
        if (!file) return;
        const ext = getExtension(file.name);
        const isVideo = BACKGROUND_VIDEO_EXTENSIONS.includes(ext);
        const isImage = BACKGROUND_IMAGE_EXTENSIONS.includes(ext);
        if (!isVideo && !isImage) {
            doFileInvalid();
            backgroundFileInput.value = '';
            return;
        }
        const reader = new FileReader();
        reader.onload = () => {
            appSettings.backgroundImagePath = reader.result;
            appSettings.backgroundMediaType = isVideo ? 'video' : 'image';
            saveAppSettings(appSettings);
            applyBackgroundMedia();
        };
        reader.onerror = () => {
            doFileInvalid();
        };
        reader.readAsDataURL(file);
        backgroundFileInput.value = '';
    });

    backgroundRemoveButton?.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        appSettings.backgroundImagePath = null;
        appSettings.backgroundMediaType = 'image';
        saveAppSettings(appSettings);
        applyBackgroundMedia();
    }, { capture: true });

    backgroundRemoveButton?.addEventListener('keydown', (e) => {
        if (e.key !== 'Enter' && e.key !== ' ') return;
        e.preventDefault();
        e.stopPropagation();
        backgroundRemoveButton.click();
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
setupQrConverter();
setupInputClearButtons();

let openDropdownState = null;

document.addEventListener('mousedown', (e) => {
    if (e.button === 1) e.preventDefault();
    if (!openDropdownState) return;
    const { toggle, popup, close } = openDropdownState;
    if (e.button === 1) {
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
    return target.closest('.switch')
        || target.closest('.autoclicker-input')?.closest('.autoclicker-input-wrap')
        || target;
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

let usingKeyboard = false;

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

const appDialog = document.getElementById('app-dialog');
const appDialogTitle = document.getElementById('app-dialog-title');
const appDialogMessage = document.getElementById('app-dialog-message');
const appDialogFooter = document.getElementById('app-dialog-footer');
const appDialogBlocked = [document.querySelector('.window'), document.querySelector('.sidebar')];
let appDialogButtons = [];
let appDialogCancelValue = null;
let appDialogResolve = null;
let appDialogReturnFocus = null;

function showDialog({ title, message, buttons, cancelValue = 'cancel' }) {
    if (!appDialog) return Promise.resolve(cancelValue);
    closeDialog(appDialogCancelValue);
    openDropdownState?.close();
    appDialogTitle.textContent = title;
    appDialogMessage.textContent = message;
    appDialogButtons = buttons.map(({ label, value, featured }) => {
        const button = document.createElement('button');
        button.className = featured ? 'app-dialog-button featured' : 'app-dialog-button';
        button.textContent = label;
        button.addEventListener('click', () => closeDialog(value));
        return button;
    });
    appDialogFooter.replaceChildren(...appDialogButtons);
    appDialogCancelValue = cancelValue;
    appDialogReturnFocus = document.activeElement;
    appDialogBlocked.forEach(el => el?.setAttribute('inert', ''));
    appDialog.classList.add('show');
    const initial = appDialogButtons[buttons.findIndex(b => b.focus)] ?? appDialogButtons[0];
    initial?.focus({ preventScroll: true });
    return new Promise(resolve => { appDialogResolve = resolve; });
}

function closeDialog(value) {
    if (!appDialogResolve) return;
    const resolve = appDialogResolve;
    appDialogResolve = null;
    appDialog.classList.remove('show');
    appDialogBlocked.forEach(el => el?.removeAttribute('inert'));
    appDialogReturnFocus?.focus?.({ preventScroll: true });
    appDialogReturnFocus = null;
    resolve(value);
}

appDialog?.addEventListener('mousedown', (e) => {
    if (!e.target.closest('.app-dialog-button')) e.preventDefault();
});

document.addEventListener('keydown', (e) => {
    if (!appDialogResolve) return;
    if (e.key === 'Escape') {
        e.preventDefault();
        e.stopPropagation();
        closeDialog(appDialogCancelValue);
    } else if (e.key === 'Tab') {
        e.preventDefault();
        e.stopPropagation();
        const count = appDialogButtons.length;
        const index = appDialogButtons.indexOf(document.activeElement);
        const next = index < 0
            ? (e.shiftKey ? count - 1 : 0)
            : (index + (e.shiftKey ? count - 1 : 1)) % count;
        appDialogButtons[next].focus();
    }
}, true);

const TOOLTIP_DELAY = 333;
const TOOLTIP_OFFSET = 20;
const TOOLTIP_CENTER_Y = 6;
const TOOLTIP_EDGE = 4;
const TOOLTIP_FLIP_MARGIN = 8;
const TOOLTIP_FOLLOW_MS = 40;
const TOOLTIP_FLIP_MS = 70;
const TOOLTIP_FLIP_DURATION = 350;
const TOOLTIP_FADE_MS = 120;

const tooltip = document.createElement('div');
tooltip.className = 'tooltip';
tooltip.setAttribute('popover', 'manual');
tooltip.setAttribute('role', 'tooltip');
document.body.appendChild(tooltip);

let tooltipTarget = null;
let tooltipTimer = null;
let tooltipHideTimer = null;
let tooltipFrame = null;
let tooltipLastFrame = 0;
let tooltipFlipUntil = 0;
let tooltipSide = 'right';
let tooltipVisible = false;
let tooltipBlocked = false;
let tooltipDragging = false;
let tooltipMouseX = 0;
let tooltipMouseY = 0;
let tooltipX = 0;
let tooltipY = 0;

function isSliderTooltip(el) {
    return el.matches('input[type="range"]') || !!el.querySelector('input[type="range"]');
}

function tooltipDestination() {
    const width = tooltip.offsetWidth;
    const height = tooltip.offsetHeight;
    const margin = tooltipSide === 'left' ? TOOLTIP_FLIP_MARGIN : 0;
    const side = tooltipMouseX + TOOLTIP_OFFSET + width <= window.innerWidth - TOOLTIP_EDGE - margin ? 'right' : 'left';
    if (side !== tooltipSide) {
        tooltipSide = side;
        tooltipFlipUntil = performance.now() + TOOLTIP_FLIP_DURATION;
    }
    const x = side === 'right' ? tooltipMouseX + TOOLTIP_OFFSET : tooltipMouseX - TOOLTIP_OFFSET - width;
    const y = Math.min(
        Math.max(tooltipMouseY + TOOLTIP_CENTER_Y - height / 2, TOOLTIP_EDGE),
        window.innerHeight - TOOLTIP_EDGE - height
    );
    return { x: Math.max(x, TOOLTIP_EDGE), y };
}

function renderTooltipPosition() {
    tooltip.style.transform = `translate3d(${tooltipX}px, ${tooltipY}px, 0)`;
}

function stepTooltip(now) {
    tooltipFrame = null;
    if (!tooltipVisible) return;
    const text = tooltipTarget?.getAttribute('tooltip');
    if (!text) return hideTooltip();
    if (tooltip.textContent !== text) tooltip.textContent = text;

    const dt = Math.min(now - tooltipLastFrame, 100);
    tooltipLastFrame = now;
    const destination = tooltipDestination();
    const smoothing = now < tooltipFlipUntil ? TOOLTIP_FLIP_MS : TOOLTIP_FOLLOW_MS;
    const k = 1 - Math.exp(-dt / smoothing);
    tooltipX += (destination.x - tooltipX) * k;
    tooltipY += (destination.y - tooltipY) * k;
    renderTooltipPosition();

    const settled = Math.abs(destination.x - tooltipX) < 0.1 && Math.abs(destination.y - tooltipY) < 0.1;
    if (!settled || tooltipDragging) tooltipFrame = requestAnimationFrame(stepTooltip);
}

function kickTooltip() {
    if (!tooltipVisible || tooltipFrame !== null) return;
    tooltipLastFrame = performance.now();
    tooltipFrame = requestAnimationFrame(stepTooltip);
}

function showTooltip() {
    clearTimeout(tooltipTimer);
    tooltipTimer = null;
    const text = tooltipTarget?.getAttribute('tooltip');
    if (!text || tooltipBlocked) return;
    clearTimeout(tooltipHideTimer);
    tooltipHideTimer = null;
    tooltip.textContent = text;
    if (!tooltip.matches(':popover-open')) tooltip.showPopover();
    tooltipSide = 'right';
    const start = tooltipDestination();
    tooltipFlipUntil = 0;
    tooltipX = start.x;
    tooltipY = start.y;
    renderTooltipPosition();
    tooltipVisible = true;
    tooltip.classList.add('show');
    kickTooltip();
}

function hideTooltip() {
    clearTimeout(tooltipTimer);
    tooltipTimer = null;
    if (!tooltipVisible) return;
    tooltipVisible = false;
    tooltip.classList.remove('show');
    clearTimeout(tooltipHideTimer);
    tooltipHideTimer = setTimeout(() => {
        tooltipHideTimer = null;
        if (!tooltipVisible && tooltip.matches(':popover-open')) tooltip.hidePopover();
    }, TOOLTIP_FADE_MS);
}

function retargetTooltip(target) {
    hideTooltip();
    tooltipTarget = target;
    tooltipBlocked = false;
    if (target) tooltipTimer = setTimeout(showTooltip, TOOLTIP_DELAY);
}

function blockTooltip() {
    hideTooltip();
    tooltipBlocked = true;
}

function endTooltipDrag(e) {
    if (!tooltipDragging) return;
    tooltipDragging = false;
    const under = document.elementFromPoint(e.clientX, e.clientY)?.closest('[tooltip]') ?? null;
    if (under !== tooltipTarget) retargetTooltip(under);
}

document.addEventListener('pointermove', (e) => {
    tooltipMouseX = e.clientX;
    tooltipMouseY = e.clientY;
    if (tooltipDragging) return kickTooltip();
    const target = e.target.closest?.('[tooltip]') ?? null;
    if (target !== tooltipTarget) return retargetTooltip(target);
    kickTooltip();
});

document.addEventListener('pointerdown', (e) => {
    const target = e.target.closest?.('[tooltip]');
    if (!target || !isSliderTooltip(target)) return blockTooltip();
    tooltipMouseX = e.clientX;
    tooltipMouseY = e.clientY;
    tooltipTarget = target;
    tooltipBlocked = false;
    tooltipDragging = true;
    if (tooltipVisible) kickTooltip();
    else showTooltip();
}, true);

document.addEventListener('pointerup', endTooltipDrag, true);
document.addEventListener('pointercancel', endTooltipDrag, true);
document.addEventListener('input', kickTooltip, true);

document.addEventListener('mouseout', (e) => {
    if (e.relatedTarget || tooltipDragging) return;
    hideTooltip();
    tooltipTarget = null;
});

document.addEventListener('keydown', blockTooltip, true);
document.addEventListener('wheel', blockTooltip, { capture: true, passive: true });
window.addEventListener('blur', () => {
    tooltipDragging = false;
    blockTooltip();
});

function initCustomDropdown(select, trigger) {
    if (select.dataset.customized) return;
    select.dataset.customized = '1';
    select.setAttribute('aria-hidden', 'true');
    select.tabIndex = -1;

    let label = null;
    const toggle = trigger || document.createElement('button');

    if (!trigger) {
        toggle.type = 'button';
        toggle.className = 'dropdown-toggle';
        toggle.setAttribute('role', 'combobox');

        label = document.createElement('span');
        label.className = 'dropdown-toggle-label';
        toggle.appendChild(label);

        select.insertAdjacentElement('afterend', toggle);
    }
    toggle.setAttribute('aria-haspopup', 'listbox');
    toggle.setAttribute('aria-expanded', 'false');

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
        if (label) label.textContent = select.options[select.selectedIndex]?.textContent || '';
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

    function clampPopupToWindowTop() {
        const popupRect = popup.getBoundingClientRect();
        if (popupRect.top < 35) {
            const currentTop = parseFloat(popup.style.top) || 0;
            popup.style.top = `${currentTop + (35 - popupRect.top)}px`;
        }
    }

    function openPopup() {
        if (isOpen) return;
        clearCloseCleanup();
        isOpen = true;
        buildOptions();
        positionPopup();
        if (!popup.matches(':popover-open')) popup.showPopover();
        clampPopupToWindowTop();
        if (warnBox.matches(':popover-open')) promoteWarn();
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

document.querySelectorAll('select.dropdown').forEach((select) => initCustomDropdown(select));



const AC_SETTINGS_KEY = 'ftools:autoclicker-settings';

const acClickTypeSelect = document.getElementById('ac-button');
const acCpsInput = document.getElementById('ac-cps');
const acHoldTimeInput = document.getElementById('ac-hold-time');

let acRestoringSettings = true;

function loadAcSettings() {
    try {
        const raw = localStorage.getItem(AC_SETTINGS_KEY);
        return raw ? JSON.parse(raw) : {};
    } catch (err) {
        console.error('Could not load autoclicker settings:', err);
        return {};
    }
}

function saveAcSettings() {
    if (acRestoringSettings) return;
    try {
        localStorage.setItem(AC_SETTINGS_KEY, JSON.stringify({
            clickType: acClickTypeSelect ? acClickTypeSelect.value : undefined,
            clickMode: acClickMode,
            clickKey: acClickKeyValue,
            actions: acActionList,
            actionListFolded: acRowFolded,
            cps: acCpsInput ? acCpsInput.value : undefined,
            holdTime: acHoldTimeInput ? acHoldTimeInput.value : undefined,
            toggleKey: acToggleKeyValue
        }));
    } catch (err) {
        console.error('Could not save autoclicker settings:', err);
    }
}

if (acClickTypeSelect) acClickTypeSelect.addEventListener('change', saveAcSettings);



const acModeMouseBtn = document.getElementById('ac-mode-mouse');
const acModeKeyboardBtn = document.getElementById('ac-mode-keyboard');
const acModeMultipleBtn = document.getElementById('ac-mode-multiple');
const acModeSwitch = document.getElementById('ac-mode-switch');
const acClickTypeRow = document.getElementById('ac-click-type-row');
const acMouseButtonRow = document.getElementById('ac-mouse-button-row');
const acClickKeyRow = document.getElementById('ac-click-key-row');
const acActionListRow = document.getElementById('ac-action-list-row');
const acActionListEl = document.getElementById('ac-action-list');
const acActionAddBtn = document.getElementById('ac-action-add');
const acActionTypeSelect = document.getElementById('ac-action-type');
if (acActionTypeSelect && acActionAddBtn) {
    initCustomDropdown(acActionTypeSelect, acActionAddBtn);
    acActionTypeSelect.selectedIndex = -1;
}
if (acActionTypeSelect) acActionTypeSelect.addEventListener('change', () => {
    createAcAction(acActionTypeSelect.value);
    acActionTypeSelect.selectedIndex = -1;
});

const AC_MAX_ACTIONS = 15;
let acActionList = [];
let acActionDetecting = null;
let acActionPendingModifiers = [];
let acActionModifierTimer = null;
let acActionInitialListenTimer = null;

let acClickMode = 'mouse';
let acRowFolded = false;

function clearRowAnimation(row) {
    if (row._acAnimCleanup) {
        row.removeEventListener('transitionend', row._acAnimCleanup);
        row._acAnimCleanup = null;
    }
}

function resetRowAnimationStyles(row) {
    if (!row) return;
    clearRowAnimation(row);
    row.style.transition = '';
    row.style.height = '';
    row.style.marginTop = '';
    row.style.overflow = '';
    row.style.boxSizing = '';
}

function expandLinkedRow(row, onDone) {
    if (!row) return;
    clearRowAnimation(row);
    row.classList.remove('hidden');
    row.style.transition = 'none';
    row.style.overflow = 'hidden';
    row.style.marginTop = '0px';
    row.style.height = '';
    row.style.borderTopWidth = '';
    row.style.borderRightWidth = '';
    row.style.borderBottomWidth = '';
    row.style.borderLeftWidth = '';
    const targetHeight = row.getBoundingClientRect().height;
    row.style.height = '0px';
    row.style.borderTopWidth = '0px';
    row.style.borderRightWidth = '0px';
    row.style.borderBottomWidth = '0px';
    row.style.borderLeftWidth = '0px';
    void row.offsetHeight;
    row.style.transition = 'height 0.15s ease-in, margin-top 0.15s ease-in, border-width 0.15s ease-in';
    row.style.height = targetHeight + 'px';
    row.style.marginTop = '-10px';
    row.style.borderTopWidth = '5px';
    row.style.borderRightWidth = '1px';
    row.style.borderBottomWidth = '1px';
    row.style.borderLeftWidth = '1px';
    const cleanup = (e) => {
        if (e && e.target !== row) return;
        row.removeEventListener('transitionend', cleanup);
        row._acAnimCleanup = null;
        row.style.transition = '';
        row.style.height = '';
        row.style.marginTop = '';
        row.style.overflow = '';
        row.style.borderTopWidth = '';
        row.style.borderRightWidth = '';
        row.style.borderBottomWidth = '';
        row.style.borderLeftWidth = '';
        if (onDone) onDone();
    };
    row._acAnimCleanup = cleanup;
    row.addEventListener('transitionend', cleanup);
}

function collapseLinkedRow(row, onDone) {
    if (!row) return;
    clearRowAnimation(row);
    const startHeight = row.getBoundingClientRect().height;
    row.style.boxSizing = 'border-box';
    row.style.overflow = 'hidden';
    row.style.transition = 'none';
    row.style.height = startHeight + 'px';
    row.style.marginTop = '-10px';
    row.style.borderTopWidth = '5px';
    row.style.borderRightWidth = '1px';
    row.style.borderBottomWidth = '1px';
    row.style.borderLeftWidth = '1px';
    void row.offsetHeight;
    row.style.transition = 'height 0.15s ease-in, margin-top 0.15s ease-in, border-width 0.15s ease-in';
    row.style.height = '0px';
    row.style.marginTop = '0px';
    row.style.borderTopWidth = '0px';
    row.style.borderRightWidth = '0px';
    row.style.borderBottomWidth = '0px';
    row.style.borderLeftWidth = '0px';
    const cleanup = (e) => {
        if (e && e.target !== row) return;
        row.removeEventListener('transitionend', cleanup);
        row._acAnimCleanup = null;
        row.classList.add('hidden');
        row.style.transition = '';
        row.style.height = '';
        row.style.marginTop = '';
        row.style.overflow = '';
        row.style.boxSizing = '';
        row.style.borderTopWidth = '';
        row.style.borderRightWidth = '';
        row.style.borderBottomWidth = '';
        row.style.borderLeftWidth = '';
        if (onDone) onDone();
    };
    row._acAnimCleanup = cleanup;
    row.addEventListener('transitionend', cleanup);
}

function setAcClickMode(mode, { save = true, resetFold = true } = {}) {
    const normalizedMode = mode === 'keyboard' || mode === 'multiple' ? mode : 'mouse';
    const modeChanged = normalizedMode !== acClickMode;
    const cameFromFolded = modeChanged && acRowFolded;
    if (resetFold && modeChanged) acRowFolded = false;
    acClickMode = normalizedMode;

    if (acModeMouseBtn) {
        acModeMouseBtn.classList.toggle('selected', acClickMode === 'mouse');
        acModeMouseBtn.setAttribute('aria-selected', acClickMode === 'mouse' ? 'true' : 'false');
    }
    if (acModeKeyboardBtn) {
        acModeKeyboardBtn.classList.toggle('selected', acClickMode === 'keyboard');
        acModeKeyboardBtn.setAttribute('aria-selected', acClickMode === 'keyboard' ? 'true' : 'false');
    }
    if (acModeMultipleBtn) {
        acModeMultipleBtn.classList.toggle('selected', acClickMode === 'multiple');
        acModeMultipleBtn.setAttribute('aria-selected', acClickMode === 'multiple' ? 'true' : 'false');
    }

    if (acClickMode === 'multiple') renderAcActionList();

    const targetRow = acClickMode === 'mouse' ? acMouseButtonRow : acClickMode === 'keyboard' ? acClickKeyRow : acActionListRow;

    if (acMouseButtonRow) acMouseButtonRow.classList.toggle('hidden', acClickMode !== 'mouse' || acRowFolded);
    if (acClickKeyRow) acClickKeyRow.classList.toggle('hidden', acClickMode !== 'keyboard' || acRowFolded);
    if (acActionListRow) {
        const shouldShowActionList = acClickMode === 'multiple' && !acRowFolded;
        acActionListRow.classList.toggle('hidden', !shouldShowActionList);
        resetRowAnimationStyles(acActionListRow);
    }
    resetRowAnimationStyles(acMouseButtonRow);
    resetRowAnimationStyles(acClickKeyRow);

    if (targetRow) {
        if (cameFromFolded) expandLinkedRow(targetRow);
        else resetRowAnimationStyles(targetRow);
    }

    if (save) saveAcSettings();
}

if (acModeMouseBtn) acModeMouseBtn.addEventListener('click', () => setAcClickMode('mouse'));
if (acModeKeyboardBtn) acModeKeyboardBtn.addEventListener('click', () => setAcClickMode('keyboard'));
if (acModeMultipleBtn) acModeMultipleBtn.addEventListener('click', () => setAcClickMode('multiple'));

if (acClickTypeRow) {
    acClickTypeRow.addEventListener('click', (e) => {
        if (acModeSwitch && acModeSwitch.contains(e.target)) return;
        const linkedRow = acClickMode === 'mouse' ? acMouseButtonRow : acClickMode === 'keyboard' ? acClickKeyRow : acActionListRow;
        if (!linkedRow) return;
        acRowFolded = !acRowFolded;
        if (acRowFolded) collapseLinkedRow(linkedRow);
        else expandLinkedRow(linkedRow);
        saveAcSettings();
    });
}

function updateAcActionAddButton() {
    if (!acActionAddBtn) return;
    const full = acActionList.length >= AC_MAX_ACTIONS;
    acActionAddBtn.disabled = full;
}

function removeAcAction(actionId) {
    acActionList = acActionList.filter((action) => action.id !== actionId);
    renderAcActionList();
    saveAcSettings();
}

function startAcActionDetection(action, button) {
    stopAcActionDetection();
    acActionDetecting = { action, button, previous: action.key || null };
    button.classList.add('listening');
    button.textContent = 'Press any key...';
    document.addEventListener('keydown', onAcActionDetectKeydown, true);
    document.addEventListener('mousedown', onAcActionDetectMouseDown, true);
    acActionInitialListenTimer = setTimeout(() => {
        acActionInitialListenTimer = null;
        stopAcActionDetection(true);
    }, AC_MODIFIER_INITIAL_WAIT);
}

function clearAcActionModifierWait() {
    if (acActionModifierTimer) {
        clearTimeout(acActionModifierTimer);
        acActionModifierTimer = null;
    }
    acActionPendingModifiers = [];
}

function armAcActionModifierWait(duration) {
    if (acActionModifierTimer) clearTimeout(acActionModifierTimer);
    if (acActionDetecting?.button) acActionDetecting.button.textContent = buildAcModifierCombo(acActionPendingModifiers);
    acActionModifierTimer = setTimeout(() => {
        const combo = buildAcModifierCombo(acActionPendingModifiers);
        clearAcActionModifierWait();
        finalizeAcActionKey(combo);
    }, duration);
}

function finalizeAcActionKey(displayValue) {
    if (!acActionDetecting) return;
    if (displayValue !== null && displayValue === acToggleKeyValue) {
        triggerWarn('ac-action-key-conflict');
        stopAcActionDetection(true);
        return;
    }
    acActionDetecting.action.key = displayValue;
    stopAcActionDetection(false);
    renderAcActionList();
    saveAcSettings();
}

function stopAcActionDetection(restore = true) {
    if (!acActionDetecting) return;
    const { button, previous } = acActionDetecting;
    clearAcActionModifierWait();
    if (acActionInitialListenTimer) {
        clearTimeout(acActionInitialListenTimer);
        acActionInitialListenTimer = null;
    }
    button?.classList.remove('listening');
    if (restore && button) button.textContent = previous || 'None';
    document.removeEventListener('keydown', onAcActionDetectKeydown, true);
    document.removeEventListener('mousedown', onAcActionDetectMouseDown, true);
    acActionDetecting = null;
}

function onAcActionDetectMouseDown(e) {
    if (!acActionDetecting) return;
    if (!acActionDetecting.button?.contains(e.target)) stopAcActionDetection(true);
}

function onAcActionDetectKeydown(e) {
    if (!acActionDetecting) return;
    if (acActionInitialListenTimer) {
        clearTimeout(acActionInitialListenTimer);
        acActionInitialListenTimer = null;
    }
    e.preventDefault();
    e.stopPropagation();

    const isCancelKey = e.key === 'Escape' || e.key === 'Meta';

    if (acActionPendingModifiers.length) {
        if (e.repeat) return;
        if (isCancelKey) {
            clearAcActionModifierWait();
            finalizeAcActionKey(null);
            return;
        }
        if (AC_MODIFIER_KEYS.includes(e.key)) {
            if (!acActionPendingModifiers.includes(e.key)) acActionPendingModifiers.push(e.key);
            armAcActionModifierWait(AC_MODIFIER_STACK_WAIT);
            return;
        }
        const combo = buildAcModifierCombo(acActionPendingModifiers, getKeyDisplayName(e));
        clearAcActionModifierWait();
        finalizeAcActionKey(combo);
        return;
    }

    if (isCancelKey) {
        finalizeAcActionKey(null);
        return;
    }

    if (e.repeat) return;

    if (AC_MODIFIER_KEYS.includes(e.key)) {
        acActionPendingModifiers = [e.key];
        armAcActionModifierWait(AC_MODIFIER_INITIAL_WAIT);
        return;
    }

    finalizeAcActionKey(getKeyDisplayName(e));
}

function renderAcActionList() {
    if (!acActionListEl) return;
    acActionListEl.innerHTML = '';
    acActionList.forEach((action) => {
        const item = document.createElement('div');
        item.className = 'ac-action-item';
        const icon = document.createElement('i');
        icon.className = action.type === 'keyboard' ? 'codicon codicon-keyboard' : 'icon icon-mouse';
        const type = document.createElement('span');
        type.className = 'ac-action-type';
        type.textContent = action.type === 'keyboard' ? 'Keyboard' : 'Mouse';
        item.append(icon, type);

        if (action.type === 'keyboard') {
            const keyButton = document.createElement('button');
            keyButton.type = 'button';
            keyButton.className = 'ac-key-detect';
            keyButton.textContent = action.key || 'None';
            keyButton.addEventListener('click', () => startAcActionDetection(action, keyButton));
            item.appendChild(keyButton);
        } else {
            const select = document.createElement('select');
            select.className = 'dropdown';
            select.innerHTML = '<option value="left">Left</option><option value="right">Right</option><option value="middle">Middle</option>';
            select.value = action.mouseButton || 'left';
            select.addEventListener('change', () => { action.mouseButton = select.value; saveAcSettings(); });
            item.appendChild(select);
            initCustomDropdown(select);
        }

        const remove = document.createElement('button');
        remove.type = 'button';
        remove.className = 'ac-action-remove';
        remove.setAttribute('aria-label', 'Remove action');
        remove.setAttribute('tooltip', 'Remove action');
        remove.innerHTML = '<svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" /></svg>';
        remove.addEventListener('click', () => removeAcAction(action.id));
        item.appendChild(remove);
        acActionListEl.appendChild(item);
    });
    updateAcActionAddButton();
    if (acActionListRow) acActionListRow.toggleAttribute('has-item', acActionList.length > 0);
}

function createAcAction(type) {
    if (acActionList.length >= AC_MAX_ACTIONS) return;
    acActionList.unshift({
        id: (typeof crypto !== 'undefined' && crypto.randomUUID) ? crypto.randomUUID() : `action-${Date.now()}-${Math.random().toString(16).slice(2)}`,
        type: type === 'keyboard' ? 'keyboard' : 'mouse',
        mouseButton: 'left',
        key: null
    });
    renderAcActionList();
    saveAcSettings();
}


const acToggleKeyBtn = document.getElementById('ac-toggle-key');
const acToggleKeyLabel = document.getElementById('ac-toggle-key-label');
const acHotkeyHint = document.getElementById('ac-hotkey-hint');

const KEY_DISPLAY_NAMES = {
    ' ': 'Space',
    'Control': 'Ctrl',
    'Delete': 'Del',
    'Backspace': 'Backspace',
    'Enter': 'Enter',
    'Tab': 'Tab',
    'CapsLock': 'Caps Lock',
    'PageUp': 'PgUp',
    'PageDown': 'PgDn',
    'Home': 'Home',
    'End': 'End',
    'Insert': 'Insert',
    'ArrowUp': 'Up Arrow',
    'ArrowDown': 'Down Arrow',
    'ArrowLeft': 'Left Arrow',
    'ArrowRight': 'Right Arrow',
    ',': 'Comma',
    '.': 'Period',
    ';': 'Semicolon',
    "'": 'Quote',
    '/': 'Slash',
    '\\': 'Backslash',
    '[': '[',
    ']': ']',
    '-': '-',
    '=': '=',
    '`': '`',
};

function getKeyDisplayName(e) {
    const key = e.key;
    if (/^F([1-9]|1[0-9]|2[0-4])$/.test(key)) return key;
    if (/^[a-zA-Z]$/.test(key)) return key.toUpperCase();
    if (/^[0-9]$/.test(key)) return key;
    if (KEY_DISPLAY_NAMES[key]) return KEY_DISPLAY_NAMES[key];
    return key.length === 1 ? key.toUpperCase() : key;
}

// Converts a toggle-key display string (e.g. "Ctrl + Shift + F6") into a
// Tauri global-shortcut accelerator string (e.g. "Ctrl+Shift+F6"), so the
// same key can be registered as a real OS-level hotkey and still work when
// fTools isn't the focused window. Returns null when the combo can't be
// expressed as a global hotkey (e.g. a modifier held alone with no key), in
// which case the toggle still works normally whenever fTools is focused.
const AC_ACCELERATOR_MODIFIER_MAP = { Ctrl: 'Ctrl', Shift: 'Shift', Alt: 'Alt' };
const AC_ACCELERATOR_KEY_MAP = {
    'Space': 'Space',
    'Del': 'Delete',
    'Backspace': 'Backspace',
    'Enter': 'Enter',
    'Tab': 'Tab',
    'Caps Lock': 'CapsLock',
    'PgUp': 'PageUp',
    'PgDn': 'PageDown',
    'Home': 'Home',
    'End': 'End',
    'Insert': 'Insert',
    'Up Arrow': 'ArrowUp',
    'Down Arrow': 'ArrowDown',
    'Left Arrow': 'ArrowLeft',
    'Right Arrow': 'ArrowRight',
    'Comma': 'Comma',
    'Period': 'Period',
    'Semicolon': 'Semicolon',
    'Quote': 'Quote',
    'Slash': 'Slash',
    'Backslash': 'Backslash',
    '[': 'BracketLeft',
    ']': 'BracketRight',
    '-': 'Minus',
    '=': 'Equal',
    '`': 'Backquote',
};

function acKeyDisplayToAcceleratorKey(display) {
    if (/^F([1-9]|1[0-9]|2[0-4])$/.test(display)) return display;
    if (/^[A-Z]$/.test(display)) return display;
    if (/^[0-9]$/.test(display)) return display;
    return AC_ACCELERATOR_KEY_MAP[display] || null;
}

function acDisplayToAccelerator(displayValue) {
    if (!displayValue) return null;
    const parts = displayValue.split(' + ');
    const mainDisplay = parts[parts.length - 1];
    if (AC_ACCELERATOR_MODIFIER_MAP[mainDisplay]) return null;
    const acceleratorKey = acKeyDisplayToAcceleratorKey(mainDisplay);
    if (!acceleratorKey) return null;
    const modifierKeys = parts.slice(0, -1).map((p) => AC_ACCELERATOR_MODIFIER_MAP[p]).filter(Boolean);
    return [...modifierKeys, acceleratorKey].join('+');
}

const acIsTauriEnv = '__TAURI_INTERNALS__' in window;
let acRegisteredAccelerator = null;
// Declared here (rather than down by the rest of the click-loop state)
// because acShouldHotkeyBeGlobal reads it, and that can run synchronously
// via setAcToggleKey(null) below before this script reaches that section.
let acRunning = false;

// Registers acToggleKeyValue as a real OS-level hotkey via the
// global-shortcut plugin, so it fires even when fTools isn't focused.
// Always unregisters whatever was registered before, so changing the
// toggle key (or clearing it) never leaves a stale hotkey behind.
//
// A registered global hotkey fully swallows that key system-wide, even
// when the person is just trying to type it into an unrelated field, so
// this only actually registers it while acShouldHotkeyBeGlobal() says it's
// needed (see that function for exactly when). The rest of the time the
// key is left completely unregistered, so it behaves like any other key.
async function syncAcGlobalHotkey() {
    if (!acIsTauriEnv || !window.__TAURI__?.globalShortcut) return;
    const { register, unregister } = window.__TAURI__.globalShortcut;
    if (acRegisteredAccelerator) {
        try {
            await unregister(acRegisteredAccelerator);
        } catch (err) {
            console.error('Could not unregister the previous autoclicker hotkey:', err);
        }
        acRegisteredAccelerator = null;
    }
    if (!acShouldHotkeyBeGlobal()) return;
    const accelerator = acDisplayToAccelerator(acToggleKeyValue);
    if (!accelerator) return;
    const handleHotkeyEvent = (event) => {
        if (event.state !== 'Pressed') return;
        if (acListeningForKey) return;
        acHandleHotkeyToggle();
    };
    try {
        await register(accelerator, handleHotkeyEvent);
        acRegisteredAccelerator = accelerator;
    } catch (err) {
        // A frontend-only reload (dev hot reload) can leave this exact
        // accelerator registered on the Rust side from the previous session,
        // even though acRegisteredAccelerator here starts out null and has
        // nothing to unregister above. Clear it and retry once before
        // giving up.
        try {
            await unregister(accelerator);
            await register(accelerator, handleHotkeyEvent);
            acRegisteredAccelerator = accelerator;
        } catch (retryErr) {
            console.error('Could not register the autoclicker hotkey globally:', retryErr);
        }
    }
}

// The toggle key only needs OS-wide reach in two cases: the clicker is
// already running (so Stop always works, from any tab or app), or the
// person is sitting on the Auto Clicker tab right now (so Start works
// immediately). Otherwise (idle and looking at something else entirely)
// there's nothing for the hotkey to do, so it's left unregistered rather
// than silently eating that key everywhere in the OS.
function acShouldHotkeyBeGlobal() {
    return acRunning || currentPanelId === 'autoclicker';
}

let acToggleKeyValue = null;
let acListeningForKey = false;

function updateHotkeyHint() {
    if (!acHotkeyHint) return;
    if (acToggleKeyValue) {
        acHotkeyHint.textContent = acToggleKeyValue;
        acHotkeyHint.style.display = '';
    } else {
        acHotkeyHint.textContent = '';
        acHotkeyHint.style.display = 'none';
    }
}

function setAcToggleKey(displayValue) {
    acToggleKeyValue = displayValue;
    if (acToggleKeyLabel) acToggleKeyLabel.textContent = displayValue || 'None';
    updateHotkeyHint();
    saveAcSettings();
    syncAcGlobalHotkey();
}

const AC_MODIFIER_KEYS = ['Control', 'Shift', 'Alt'];
const AC_MODIFIER_ORDER = ['Control', 'Shift', 'Alt'];
const AC_MODIFIER_INITIAL_WAIT = 1200;
const AC_MODIFIER_STACK_WAIT = 1200;

let acPendingModifiers = [];
let acModifierTimer = null;

function buildAcModifierCombo(modifiers, extraKeyDisplay) {
    const parts = AC_MODIFIER_ORDER
        .filter((mod) => modifiers.includes(mod))
        .map((mod) => KEY_DISPLAY_NAMES[mod] || mod);
    if (extraKeyDisplay) parts.push(extraKeyDisplay);
    return parts.join(' + ');
}

function clearAcModifierWait() {
    if (acModifierTimer) {
        clearTimeout(acModifierTimer);
        acModifierTimer = null;
    }
    acPendingModifiers = [];
}

function finalizeAcKey(displayValue) {
    if (displayValue !== null && displayValue === acClickKeyValue) {
        triggerWarn('ac-toggle-key-conflict');
        stopAcListening();
        return;
    }
    if (displayValue !== null && acActionList.some((action) => action.type === 'keyboard' && action.key === displayValue)) {
        triggerWarn('ac-toggle-action-key-conflict');
        stopAcListening();
        return;
    }
    setAcToggleKey(displayValue);
    stopAcListening();
}

let acInitialListenTimer = null;

function armAcModifierWait(duration) {
    if (acModifierTimer) clearTimeout(acModifierTimer);
    if (acToggleKeyLabel) acToggleKeyLabel.textContent = buildAcModifierCombo(acPendingModifiers);
    acModifierTimer = setTimeout(() => {
        const combo = buildAcModifierCombo(acPendingModifiers);
        clearAcModifierWait();
        finalizeAcKey(combo);
    }, duration);
}

function onAcDetectKeydown(e) {
    if (acInitialListenTimer) {
        clearTimeout(acInitialListenTimer);
        acInitialListenTimer = null;
    }
    e.preventDefault();
    e.stopPropagation();

    const isCancelKey = e.key === 'Escape' || e.key === 'Meta';

    if (acPendingModifiers.length) {
        if (e.repeat) return;
        if (isCancelKey) {
            clearAcModifierWait();
            finalizeAcKey(null);
            return;
        }
        if (AC_MODIFIER_KEYS.includes(e.key)) {
            if (!acPendingModifiers.includes(e.key)) acPendingModifiers.push(e.key);
            armAcModifierWait(AC_MODIFIER_STACK_WAIT);
            return;
        }
        const combo = buildAcModifierCombo(acPendingModifiers, getKeyDisplayName(e));
        clearAcModifierWait();
        finalizeAcKey(combo);
        return;
    }

    if (isCancelKey) {
        finalizeAcKey(null);
        return;
    }

    if (e.repeat) return;

    if (AC_MODIFIER_KEYS.includes(e.key)) {
        acPendingModifiers = [e.key];
        armAcModifierWait(AC_MODIFIER_INITIAL_WAIT);
        return;
    }

    finalizeAcKey(getKeyDisplayName(e));
}

function stopAcListening() {
    acListeningForKey = false;
    clearAcModifierWait();
    if (acInitialListenTimer) {
        clearTimeout(acInitialListenTimer);
        acInitialListenTimer = null;
    }
    if (acToggleKeyBtn) acToggleKeyBtn.classList.remove('listening');
    if (acToggleKeyLabel) acToggleKeyLabel.textContent = acToggleKeyValue || 'None';
    document.removeEventListener('keydown', onAcDetectKeydown, true);
}

if (acToggleKeyBtn) {
    acToggleKeyBtn.addEventListener('focusout', (e) => {
        if (!acToggleKeyBtn.contains(e.relatedTarget)) stopAcListening();
    });
    acToggleKeyBtn.addEventListener('click', () => {
        if (acListeningForKey) return;
        acListeningForKey = true;
        acToggleKeyBtn.classList.add('listening');
        acToggleKeyLabel.textContent = 'Press any key...';
        document.addEventListener('keydown', onAcDetectKeydown, true);
        acInitialListenTimer = setTimeout(() => {
            acInitialListenTimer = null;
            stopAcListening();
        }, AC_MODIFIER_INITIAL_WAIT);
    });
    setAcToggleKey(null);
}



const acClickKeyBtn = document.getElementById('ac-click-key');
const acClickKeyLabel = document.getElementById('ac-click-key-label');

let acClickKeyValue = null;
let acListeningForClickKey = false;

let acClickPendingModifiers = [];
let acClickModifierTimer = null;

function setAcClickKey(displayValue) {
    acClickKeyValue = displayValue;
    if (acClickKeyLabel) acClickKeyLabel.textContent = displayValue || 'None';
    saveAcSettings();
}

function clearAcClickModifierWait() {
    if (acClickModifierTimer) {
        clearTimeout(acClickModifierTimer);
        acClickModifierTimer = null;
    }
    acClickPendingModifiers = [];
}

function finalizeAcClickKey(displayValue) {
    if (displayValue !== null && displayValue === acToggleKeyValue) {
        triggerWarn('ac-click-key-conflict');
        stopAcClickListening();
        return;
    }
    setAcClickKey(displayValue);
    stopAcClickListening();
}

let acClickInitialListenTimer = null;

function armAcClickModifierWait(duration) {
    if (acClickModifierTimer) clearTimeout(acClickModifierTimer);
    if (acClickKeyLabel) acClickKeyLabel.textContent = buildAcModifierCombo(acClickPendingModifiers);
    acClickModifierTimer = setTimeout(() => {
        const combo = buildAcModifierCombo(acClickPendingModifiers);
        clearAcClickModifierWait();
        finalizeAcClickKey(combo);
    }, duration);
}

function onAcClickDetectKeydown(e) {
    if (acClickInitialListenTimer) {
        clearTimeout(acClickInitialListenTimer);
        acClickInitialListenTimer = null;
    }
    e.preventDefault();
    e.stopPropagation();

    const isCancelKey = e.key === 'Escape' || e.key === 'Meta';

    if (acClickPendingModifiers.length) {
        if (e.repeat) return;
        if (isCancelKey) {
            clearAcClickModifierWait();
            finalizeAcClickKey(null);
            return;
        }
        if (AC_MODIFIER_KEYS.includes(e.key)) {
            if (!acClickPendingModifiers.includes(e.key)) acClickPendingModifiers.push(e.key);
            armAcClickModifierWait(AC_MODIFIER_STACK_WAIT);
            return;
        }
        const combo = buildAcModifierCombo(acClickPendingModifiers, getKeyDisplayName(e));
        clearAcClickModifierWait();
        finalizeAcClickKey(combo);
        return;
    }

    if (isCancelKey) {
        finalizeAcClickKey(null);
        return;
    }

    if (e.repeat) return;

    if (AC_MODIFIER_KEYS.includes(e.key)) {
        acClickPendingModifiers = [e.key];
        armAcClickModifierWait(AC_MODIFIER_INITIAL_WAIT);
        return;
    }

    finalizeAcClickKey(getKeyDisplayName(e));
}

function stopAcClickListening() {
    acListeningForClickKey = false;
    clearAcClickModifierWait();
    if (acClickInitialListenTimer) {
        clearTimeout(acClickInitialListenTimer);
        acClickInitialListenTimer = null;
    }
    if (acClickKeyBtn) acClickKeyBtn.classList.remove('listening');
    if (acClickKeyLabel) acClickKeyLabel.textContent = acClickKeyValue || 'None';
    document.removeEventListener('keydown', onAcClickDetectKeydown, true);
}

if (acClickKeyBtn) {
    acClickKeyBtn.addEventListener('focusout', (e) => {
        if (!acClickKeyBtn.contains(e.relatedTarget)) stopAcClickListening();
    });
    acClickKeyBtn.addEventListener('click', () => {
        if (acListeningForClickKey) return;
        acListeningForClickKey = true;
        acClickKeyBtn.classList.add('listening');
        acClickKeyLabel.textContent = 'Press any key...';
        document.addEventListener('keydown', onAcClickDetectKeydown, true);
        acClickInitialListenTimer = setTimeout(() => {
            acClickInitialListenTimer = null;
            stopAcClickListening();
        }, AC_MODIFIER_INITIAL_WAIT);
    });
    setAcClickKey(null);
}



const acAutosizeCanvas = document.createElement('canvas');
const acAutosizeCtx = acAutosizeCanvas.getContext('2d');

function autosizeAcInput(input) {
    if (!input) return;
    const style = getComputedStyle(input);
    acAutosizeCtx.font = `${style.fontStyle} ${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
    const text = input.value !== '' ? input.value : (input.placeholder || '0');
    const textWidth = acAutosizeCtx.measureText(text).width;
    input.style.width = `${Math.ceil(textWidth) + 1}px`;
}

if (acCpsInput) autosizeAcInput(acCpsInput);
if (acHoldTimeInput) autosizeAcInput(acHoldTimeInput);



const AC_MAX_CPS = 1000;
const AC_CPS_WARN_THRESHOLD = 800;
let acLastValidCps = acCpsInput ? (parseInt(acCpsInput.value, 10) || 1) : 1;

function updateAcMaxHoldTime() {
    if (!acHoldTimeInput) return;
    const maxHoldTime = Math.max(1, Math.floor(1000 / acLastValidCps));
    acHoldTimeInput.max = maxHoldTime;

    if (acHoldTimeInput.value === '') return;
    let holdValue = parseInt(acHoldTimeInput.value, 10);
    if (isNaN(holdValue)) return;
    if (holdValue > maxHoldTime) {
        acHoldTimeInput.value = maxHoldTime;
        autosizeAcInput(acHoldTimeInput);
    }
}

function handleAcCpsInput() {
    if (acCpsInput.value === '') return;
    let value = parseInt(acCpsInput.value, 10);
    if (isNaN(value)) return;
    if (value <= 0) value = 1;
    if (value > AC_MAX_CPS) value = AC_MAX_CPS;
    acCpsInput.value = value;
    const previousValue = acLastValidCps;
    acLastValidCps = value;
    if (!acRestoringSettings && previousValue <= AC_CPS_WARN_THRESHOLD && value > AC_CPS_WARN_THRESHOLD) triggerWarn('ac-cps-high');
    updateAcMaxHoldTime();
    autosizeAcInput(acCpsInput);
}

function handleAcCpsChange() {
    let value = parseInt(acCpsInput.value, 10);
    if (isNaN(value) || value <= 0) value = 1;
    if (value > AC_MAX_CPS) value = AC_MAX_CPS;
    acCpsInput.value = value;
    const previousValue = acLastValidCps;
    acLastValidCps = value;
    if (!acRestoringSettings && previousValue <= AC_CPS_WARN_THRESHOLD && value > AC_CPS_WARN_THRESHOLD) triggerWarn('ac-cps-high');
    updateAcMaxHoldTime();
    autosizeAcInput(acCpsInput);
    saveAcSettings();
}

function handleAcHoldTimeInput() {
    if (acHoldTimeInput.value === '') return;
    const maxHoldTime = parseInt(acHoldTimeInput.max, 10) || 1;
    let value = parseInt(acHoldTimeInput.value, 10);
    if (isNaN(value)) return;
    if (value <= 0) value = 1;
    if (value > maxHoldTime) value = maxHoldTime;
    acHoldTimeInput.value = value;
    autosizeAcInput(acHoldTimeInput);
}

function handleAcHoldTimeChange() {
    const maxHoldTime = parseInt(acHoldTimeInput.max, 10) || 1;
    let value = parseInt(acHoldTimeInput.value, 10);
    if (isNaN(value) || value <= 0) value = 1;
    if (value > maxHoldTime) value = maxHoldTime;
    acHoldTimeInput.value = value;
    autosizeAcInput(acHoldTimeInput);
    saveAcSettings();
}

if (acCpsInput) {
    acCpsInput.addEventListener('input', handleAcCpsInput);
    acCpsInput.addEventListener('change', handleAcCpsChange);
}
if (acHoldTimeInput) {
    acHoldTimeInput.addEventListener('input', handleAcHoldTimeInput);
    acHoldTimeInput.addEventListener('change', handleAcHoldTimeChange);
}

document.querySelectorAll('.autoclicker-input-wrap').forEach((wrap) => {
    const input = wrap.querySelector('.autoclicker-input');
    if (!input) return;

    wrap.addEventListener('mousedown', (e) => {
        if (e.target === input) return;

        e.preventDefault();
        input.focus();

        const end = input.value.length;
        try {
            input.setSelectionRange(end, end);
        } catch (_) {}
    });
});

updateAcMaxHoldTime();



(function restoreAcSettings() {
    const saved = loadAcSettings();
    acRestoringSettings = true;

    if (acClickTypeSelect && saved.clickType) {
        acClickTypeSelect.value = saved.clickType;
        acClickTypeSelect.__dropdownSync?.();
    }
    acRowFolded = !!saved.actionListFolded;
    if (saved.clickMode) {
        setAcClickMode(saved.clickMode, { save: false, resetFold: false });
    }
    if (acCpsInput && saved.cps !== undefined) {
        acCpsInput.value = saved.cps;
        handleAcCpsChange();
    }
    if (acHoldTimeInput && saved.holdTime !== undefined) {
        acHoldTimeInput.value = saved.holdTime;
        handleAcHoldTimeChange();
    }
    if (saved.toggleKey) {
        setAcToggleKey(saved.toggleKey);
    }
    if (saved.clickKey) {
        setAcClickKey(saved.clickKey);
    }
    if (Array.isArray(saved.actions)) {
        acActionList = saved.actions
            .filter((action) => action && (action.type === 'mouse' || action.type === 'keyboard'))
            .slice(0, AC_MAX_ACTIONS)
            .map((action) => ({
                id: action.id || ((typeof crypto !== 'undefined' && crypto.randomUUID) ? crypto.randomUUID() : `action-${Date.now()}-${Math.random().toString(16).slice(2)}`),
                type: action.type,
                mouseButton: action.mouseButton || 'left',
                key: action.key || null
            }));
        renderAcActionList();
    }

    acRestoringSettings = false;
    saveAcSettings();
})();



const acStartBtn = document.getElementById('ac-start');
const acStartLabel = document.getElementById('ac-start-label');
const acStartIcon = acStartBtn ? acStartBtn.querySelector('i') : null;
const acTabEl = document.getElementById('autoclicker');
const acConfigEl = acTabEl ? acTabEl.querySelector('.autoclicker') : null;

let acCurrentGeneration = 0;

// Builds the list of { type, button|key } actions for the current click
// mode, in the exact shape start_autoclicker_loop expects on the Rust side.
function buildAcTickActions() {
    if (acClickMode === 'mouse') {
        return [{ type: 'mouse', button: acClickTypeSelect ? acClickTypeSelect.value : 'left' }];
    }
    if (acClickMode === 'keyboard') {
        if (!acClickKeyValue) return [];
        return [{ type: 'keyboard', key: acClickKeyValue }];
    }
    // 'multiple': fire every configured action together each tick (skipping
    // any keyboard action that was never assigned a key).
    return acActionList
        .filter((action) => action.type !== 'keyboard' || !!action.key)
        .map((action) => (action.type === 'keyboard'
            ? { type: 'keyboard', key: action.key }
            : { type: 'mouse', button: action.mouseButton || 'left' }));
}

async function startAcClicking() {
    if (acRunning) return;

    if (acClickMode === 'keyboard' && !acClickKeyValue) {
        triggerWarn('ac-no-key-set');
        return;
    }
    if (acClickMode === 'multiple') {
        if (!acActionList.length) {
            triggerWarn('ac-no-actions');
            return;
        }
        if (!acActionList.some((action) => action.type !== 'keyboard' || !!action.key)) {
            triggerWarn('ac-no-key-set');
            return;
        }
    }

    if (appSettings.closeOnFocusLoss) {
        triggerWarn('ac-close-on-focus-loss');
        return;
    }

    if (acIsTauriEnv && window.__TAURI__?.core?.invoke) {
        try {
            acCurrentGeneration = await window.__TAURI__.core.invoke('bump_autoclicker_generation');
        } catch (err) {
            console.error('Could not start the autoclicker:', err);
            return;
        }
        // Something else (e.g. the button was clicked again while awaiting)
        // already started or stopped a run; don't start a second one.
        if (acRunning) return;

        const actions = buildAcTickActions();
        const holdMs = acHoldTimeInput ? (parseInt(acHoldTimeInput.value, 10) || 1) : 1;
        const cps = acCpsInput ? (parseInt(acCpsInput.value, 10) || 1) : 1;
        const intervalMs = Math.max(1, Math.round(1000 / cps));

        // Fire-and-forget: the whole click loop (schedule tick, press, hold,
        // release, repeat) now runs entirely on the Rust side, driven by a
        // tokio timer instead of a JS setInterval. A JS timer plus one IPC
        // round trip per click was the actual ceiling on achievable CPS
        // (it capped out well under what was configured); one invoke here
        // per run removes both bottlenecks. The loop keeps running until
        // Stop bumps the generation again (see bump_autoclicker_generation
        // in stopAcClicking below).
        window.__TAURI__.core.invoke('start_autoclicker_loop', {
            actions,
            holdMs,
            intervalMs,
            generation: acCurrentGeneration,
        }).catch((err) => {
            console.error('Autoclicker loop ended unexpectedly:', err);
        });
    }

    acRunning = true;
    syncAcGlobalHotkey();

    if (acStartBtn) acStartBtn.classList.add('running');
    if (acTabEl) acTabEl.setAttribute('running', '');
    if (acConfigEl) {
        if (acConfigEl.contains(document.activeElement)) document.activeElement.blur();
        acConfigEl.inert = true;
    }
    if (acStartLabel) acStartLabel.textContent = 'Stop';
    if (acStartIcon) {
        acStartIcon.classList.remove('icon-play');
        acStartIcon.classList.add('icon-stop');
    }
}

function stopAcClicking() {
    if (!acRunning) return;
    acRunning = false;
    syncAcGlobalHotkey();

    // Invalidate this run's token right away: the Rust-side loop notices
    // within its current tick/hold wait and stops instantly instead of
    // trailing off over the next several clicks.
    if (acIsTauriEnv && window.__TAURI__?.core?.invoke) {
        window.__TAURI__.core.invoke('bump_autoclicker_generation').catch((err) => {
            console.error('Could not fully stop the autoclicker:', err);
        });
    }

    if (acStartBtn) acStartBtn.classList.remove('running');
    if (acTabEl) acTabEl.removeAttribute('running');
    if (acConfigEl) acConfigEl.inert = false;
    if (acStartLabel) acStartLabel.textContent = 'Start';
    if (acStartIcon) {
        acStartIcon.classList.remove('icon-stop');
        acStartIcon.classList.add('icon-play');
    }
}


function toggleAcRunning() {
    if (acRunning) {
        stopAcClicking();
    } else {
        startAcClicking();
    }
}

// The toggle-key hotkey fires app-wide, including when the person is on a
// completely different tab. Starting the clicker from an unrelated tab
// would be surprising (and hard to notice), so a start only goes through
// while the Auto Clicker tab is actually the one on screen. Stopping is
// always allowed regardless of tab, so the hotkey can never leave the
// clicker running with no obvious way to turn it off.
function acHandleHotkeyToggle() {
    if (acRunning) {
        stopAcClicking();
        return;
    }
    if (currentPanelId !== 'autoclicker') return;
    startAcClicking();
}

if (acStartBtn) {
    acStartBtn.addEventListener('click', toggleAcRunning);
}

function isAcTypingTarget(el) {
    if (!el) return false;
    const tag = el.tagName;
    return tag === 'INPUT' || tag === 'TEXTAREA' || el.isContentEditable;
}

document.addEventListener('keydown', (e) => {
    if (!acToggleKeyValue) return;
    if (acListeningForKey) return;
    if (e.repeat) return;
    if (!acRunning && isAcTypingTarget(e.target)) return;

    let pressedDisplay;
    if (AC_MODIFIER_KEYS.includes(e.key)) {
        pressedDisplay = buildAcModifierCombo([e.key]);
    } else {
        const heldModifiers = AC_MODIFIER_ORDER.filter((mod) => e.getModifierState(mod));
        pressedDisplay = buildAcModifierCombo(heldModifiers, getKeyDisplayName(e));
    }

    if (pressedDisplay === acToggleKeyValue) {
        e.preventDefault();
        // The global hotkey already fires for this exact key (even while
        // fTools is focused), so bail here to avoid toggling twice on one press.
        if (acIsTauriEnv && acRegisteredAccelerator) return;
        acHandleHotkeyToggle();
    }
});


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
const INDICATOR_PULL_EASE_REVERSE = 'cubic-bezier(0.7, 0, 0.84, 0)';
const INDICATOR_SETTLE_EASE_REVERSE = 'cubic-bezier(0.45, 0, 0.7, -0.4)';
const INDICATOR_CURRENT_DELAY_MS = 80;
const INDICATOR_GHOST_SETTLE_MS = INDICATOR_SETTLE_MS / 2;
const INDICATOR_GHOST_PULL_MS = INDICATOR_PULL_MS / 2;
const INDICATOR_GHOST_TOTAL_MS = INDICATOR_GHOST_SETTLE_MS + INDICATOR_GHOST_PULL_MS;
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

    if (previousButton) spawnSidebarIndicatorGhost(previousButton, movingDown);

    const leadTop = movingDown ? 0 : buttonHeight - INDICATOR_LEAD_HEIGHT;
    const stretchTop = movingDown ? 0 : buttonHeight - INDICATOR_STRETCH_HEIGHT;

    sidebarIndicator.style.transition = 'none';
    sidebarIndicator.style.top = `${leadTop}px`;
    sidebarIndicator.style.height = `${INDICATOR_LEAD_HEIGHT}px`;
    void sidebarIndicator.offsetHeight;

    setTimeout(() => {
        sidebarIndicator.style.transition = `top ${INDICATOR_PULL_MS}ms ${INDICATOR_PULL_EASE}, height ${INDICATOR_PULL_MS}ms ${INDICATOR_PULL_EASE}`;
        requestAnimationFrame(() => {
            sidebarIndicator.style.top = `${stretchTop}px`;
            sidebarIndicator.style.height = `${INDICATOR_STRETCH_HEIGHT}px`;
        });
    }, INDICATOR_CURRENT_DELAY_MS);

    indicatorSettleTimeout = setTimeout(() => {
        sidebarIndicator.style.transition = `top ${INDICATOR_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE}, height ${INDICATOR_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE}`;
        sidebarIndicator.style.top = `${restTop}px`;
        sidebarIndicator.style.height = `${INDICATOR_REST_HEIGHT}px`;
    }, INDICATOR_CURRENT_DELAY_MS + INDICATOR_PULL_MS);
}

function spawnSidebarIndicatorGhost(previousButton, movingDown) {
    const prevHeight = previousButton.offsetHeight;
    const prevRestTop = prevHeight / 2 - INDICATOR_REST_HEIGHT / 2;

    const ghostLeadTop = movingDown ? prevHeight - INDICATOR_LEAD_HEIGHT : 0;
    const ghostStretchTop = movingDown ? prevHeight - INDICATOR_STRETCH_HEIGHT : 0;

    const ghost = sidebarIndicator.cloneNode(false);
    ghost.removeAttribute('id');
    ghost.classList.add('show');
    ghost.style.transition = 'none';
    ghost.style.top = `${prevRestTop}px`;
    ghost.style.height = `${INDICATOR_REST_HEIGHT}px`;
    previousButton.appendChild(ghost);
    void ghost.offsetHeight;

    ghost.style.transition = `top ${INDICATOR_GHOST_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE_REVERSE}, height ${INDICATOR_GHOST_SETTLE_MS}ms ${INDICATOR_SETTLE_EASE_REVERSE}, opacity ${INDICATOR_GHOST_TOTAL_MS}ms linear`;
    requestAnimationFrame(() => {
        ghost.style.top = `${ghostStretchTop}px`;
        ghost.style.height = `${INDICATOR_STRETCH_HEIGHT}px`;
        ghost.style.opacity = '0';
    });

    setTimeout(() => {
        ghost.style.transition = `top ${INDICATOR_GHOST_PULL_MS}ms ${INDICATOR_PULL_EASE_REVERSE}, height ${INDICATOR_GHOST_PULL_MS}ms ${INDICATOR_PULL_EASE_REVERSE}`;
        ghost.style.top = `${ghostLeadTop}px`;
        ghost.style.height = `${INDICATOR_LEAD_HEIGHT}px`;
    }, INDICATOR_GHOST_SETTLE_MS);

    setTimeout(() => {
        ghost.remove();
    }, INDICATOR_GHOST_TOTAL_MS);
}

const TAB_TITLES = {
    fontstyler: 'Font Styler',
    colorformat: 'Color Format',
    mediafileconverter: 'Media File Converter',
    autoclicker: 'Auto Clicker',
    qrcodeconverter: 'QR Code Maker',
    settings: 'Settings',
    'sidebar-editor': 'Sidebar Editor'
};

const titlebarTitle = document.getElementById('titlebar-title');

function applyTitlebarTitle(fullTitle) {
    if (titlebarTitle) titlebarTitle.textContent = fullTitle;
    document.title = fullTitle;

    window.__TAURI__?.window?.getCurrentWindow?.()?.setTitle?.(fullTitle).catch(() => {});
}

function updateWindowTitle(panelId) {
    const panelChanged = currentPanelId !== panelId;
    currentPanelId = panelId;
    const label = TAB_TITLES[panelId] || panelId;
    applyTitlebarTitle(appSettings.hideToolInTitlebar ? 'fTools' : `fTools | ${label}`);
    if (panelChanged) syncAcGlobalHotkey();
}

function setConversionProgressTitle(percent) {
    const label = percent >= 100 ? '100%' : `${percent.toFixed(1)}%`;
    applyTitlebarTitle(appSettings.hideToolInTitlebar ? 'fTools' : `fTools | Converting a file ${label}`);
}

function suspendQrSliderAnim() {
    const thumb = document.getElementById('qr-ecl-thumb');
    const fill = document.getElementById('qr-ecl-fill');
    if (!thumb && !fill) return;
    thumb?.classList.add('no-anim');
    fill?.classList.add('no-anim');
    clearTimeout(suspendQrSliderAnim._t);
    suspendQrSliderAnim._t = setTimeout(() => {
        thumb?.classList.remove('no-anim');
        fill?.classList.remove('no-anim');
    }, 50);
}

function activatePanel(panelId) {
    const targetPanel = document.getElementById(panelId);
    if (!targetPanel) return;

    panels.forEach(panel => panel.classList.remove('active'));
    sidebarButtons.forEach(button => button.classList.remove('active'));

    if (panelId === 'qrcodeconverter') {
        suspendQrSliderAnim();
    }

    targetPanel.classList.add('active');
    const activeButtonId = panelId === 'sidebar-editor' ? 'settings' : panelId;
    const activeButton = document.getElementById(`t-${activeButtonId}`);
    activeButton?.classList.add('active');
    moveSidebarIndicator(activeButton);
    updateWindowTitle(panelId);
}

const SIDEBAR_CONFIG_KEY = 'ftools:sidebar-config';
const MAX_VISIBLE_SIDEBAR_TOOLS = 4;
const DEFAULT_HIDDEN_SIDEBAR_IDS = ['qrcodeconverter'];
const EDITABLE_SIDEBAR_IDS = Array.from(sidebarButtons)
    .map(button => button.id.replace(/^t-/, ''))
    .filter(id => id !== 'settings');

function loadSidebarConfig() {
    try {
        const raw = localStorage.getItem(SIDEBAR_CONFIG_KEY);
        const saved = raw ? JSON.parse(raw) : {};
        const savedOrder = Array.isArray(saved.order) ? saved.order.filter(id => EDITABLE_SIDEBAR_IDS.includes(id)) : [];
        const newIds = EDITABLE_SIDEBAR_IDS.filter(id => !savedOrder.includes(id));
        const order = [...savedOrder, ...newIds];
        const savedHidden = Array.isArray(saved.hidden) ? saved.hidden.filter(id => EDITABLE_SIDEBAR_IDS.includes(id)) : [];
        const hidden = [...savedHidden, ...newIds.filter(id => DEFAULT_HIDDEN_SIDEBAR_IDS.includes(id) && !savedHidden.includes(id))];
        return { order, hidden };
    } catch (err) {
        console.error('Could not load sidebar config:', err);
        return { order: [...EDITABLE_SIDEBAR_IDS], hidden: [...DEFAULT_HIDDEN_SIDEBAR_IDS] };
    }
}

function saveSidebarConfig(config) {
    try {
        localStorage.setItem(SIDEBAR_CONFIG_KEY, JSON.stringify(config));
    } catch (err) {
        console.error('Could not save sidebar config:', err);
    }
}

let sidebarConfig = loadSidebarConfig();
const sidebarContainer = document.querySelector('.sidebar');

function updateSidebarTabindexes() {
    if (!sidebarContainer) return;

    const orderedButtons = Array.from(sidebarContainer.children)
        .filter(child => child.classList.contains('sidebar-button') && child.id !== 't-settings');

    orderedButtons.forEach((button, index) => {
        button.tabIndex = index + 1;
    });

    const settingsButton = document.getElementById('t-settings');
    if (settingsButton) settingsButton.tabIndex = orderedButtons.length + 1;
}

function applySidebarConfig() {
    if (!sidebarContainer) return;

    const buttons = EDITABLE_SIDEBAR_IDS
        .map(id => document.getElementById(`t-${id}`))
        .filter(Boolean);

    const firstRects = new Map();
    buttons.forEach(button => firstRects.set(button, button.getBoundingClientRect()));

    sidebarConfig.order.forEach(id => {
        const button = document.getElementById(`t-${id}`);
        if (button) sidebarContainer.appendChild(button);
    });
    EDITABLE_SIDEBAR_IDS.forEach(id => {
        const button = document.getElementById(`t-${id}`);
        button?.classList.toggle('sidebar-button-hidden', sidebarConfig.hidden.includes(id));
    });

    updateSidebarTabindexes();

    buttons.forEach(button => {
        if (button.classList.contains('sidebar-button-hidden')) return;

        const firstRect = firstRects.get(button);
        if (firstRect.width === 0 && firstRect.height === 0) return;

        const lastRect = button.getBoundingClientRect();
        const deltaY = firstRect.top - lastRect.top;
        if (deltaY === 0) return;

        button.style.transition = 'none';
        button.style.transform = `translateX(-50%) translateY(${deltaY}px)`;
        void button.offsetHeight;
        button.style.transition = 'transform 0.15s ease';
        button.style.transform = 'translateX(-50%)';
        button.addEventListener('transitionend', () => {
            button.style.transition = '';
            button.style.transform = '';
        }, { once: true });
    });
}

applySidebarConfig();

let editorDraggingItem = null;

function getEditorDragAfterElement(container, y) {
    const items = Array.from(container.querySelectorAll('.editor-item:not(.dragging)'));
    return items.reduce((closest, child) => {
        const box = child.getBoundingClientRect();
        const offset = y - box.top - box.height / 2;
        if (offset < 0 && offset > closest.offset) return { offset, element: child };
        return closest;
    }, { offset: Number.NEGATIVE_INFINITY, element: null }).element;
}

function startEditorDrag(e, item) {
    e.preventDefault();
    editorDraggingItem = item;
    item.classList.add('dragging');
    document.body.classList.add('editor-dragging');
    const handle = e.currentTarget;
    handle.setPointerCapture?.(e.pointerId);
    const list = item.parentElement;

    function onMove(ev) {
        if (!editorDraggingItem) return;
        const afterElement = getEditorDragAfterElement(list, ev.clientY);
        if (afterElement === editorDraggingItem) return;
        if (afterElement == null && list.lastElementChild === editorDraggingItem) return;

        const others = Array.from(list.children).filter(el => el !== editorDraggingItem);
        const firstRects = new Map(others.map(el => [el, el.getBoundingClientRect()]));

        if (afterElement == null) list.appendChild(editorDraggingItem);
        else list.insertBefore(editorDraggingItem, afterElement);

        others.forEach(el => {
            const firstRect = firstRects.get(el);
            const lastRect = el.getBoundingClientRect();
            const deltaY = firstRect.top - lastRect.top;
            if (!deltaY) return;

            el.style.transition = 'none';
            el.style.transform = `translateY(${deltaY}px)`;
            void el.offsetHeight;
            el.style.transition = 'transform 0.15s ease';
            el.style.transform = '';
            el.addEventListener('transitionend', () => {
                el.style.transition = '';
                el.style.transform = '';
            }, { once: true });
        });
    }

    function onUp(ev) {
        if (!editorDraggingItem) return;
        editorDraggingItem.classList.remove('dragging');
        editorDraggingItem = null;
        document.body.classList.remove('editor-dragging');
        handle.releasePointerCapture?.(ev.pointerId);
        window.removeEventListener('pointermove', onMove);
        window.removeEventListener('pointerup', onUp);

        sidebarConfig.order = Array.from(list.children).map(child => child.dataset.id);
        saveSidebarConfig(sidebarConfig);
        applySidebarConfig();
    }

    window.addEventListener('pointermove', onMove);
    window.addEventListener('pointerup', onUp);
}

function renderEditorList() {
    const list = document.getElementById('editor-list');
    if (!list) return;
    list.innerHTML = '';

    sidebarConfig.order.forEach(id => {
        const sourceButton = document.getElementById(`t-${id}`);
        if (!sourceButton) return;

        const item = document.createElement('div');
        item.className = 'editor-item';
        item.dataset.id = id;
        if (sidebarConfig.hidden.includes(id)) item.classList.add('hidden-tool');

        const handle = document.createElement('div');
        handle.className = 'editor-handle';
        handle.setAttribute('tooltip', 'Drag to reorder');
        handle.innerHTML = '<svg width="10" height="16" viewBox="0 0 10 16" fill="currentColor"><circle cx="2.5" cy="2.5" r="1.5"/><circle cx="7.5" cy="2.5" r="1.5"/><circle cx="2.5" cy="8" r="1.5"/><circle cx="7.5" cy="8" r="1.5"/><circle cx="2.5" cy="13.5" r="1.5"/><circle cx="7.5" cy="13.5" r="1.5"/></svg>';
        handle.addEventListener('pointerdown', (e) => startEditorDrag(e, item));

        const icon = document.createElement('div');
        icon.className = 'editor-icon';
        const clonedIcon = sourceButton.querySelector('i')?.cloneNode(true);
        if (clonedIcon) {
            clonedIcon.style.fontSize = '16px';
            icon.appendChild(clonedIcon);
        }

        const label = document.createElement('div');
        label.className = 'editor-label';
        label.textContent = TAB_TITLES[id] || id;

        const toggleLabel = document.createElement('label');
        toggleLabel.className = 'switch editor-toggle';
        const toggleInput = document.createElement('input');
        toggleInput.type = 'checkbox';
        toggleInput.checked = !sidebarConfig.hidden.includes(id);
        const slider = document.createElement('span');
        slider.className = 'slider round';
        toggleLabel.append(toggleInput, slider);

        toggleInput.addEventListener('change', () => {
            const visibleCount = EDITABLE_SIDEBAR_IDS.length - sidebarConfig.hidden.length;
            if (!toggleInput.checked && visibleCount <= 1) {
                toggleInput.checked = true;
                triggerWarn('sidebar-min-tools-active');
                return;
            }
            if (toggleInput.checked && visibleCount >= MAX_VISIBLE_SIDEBAR_TOOLS) {
                toggleInput.checked = false;
                triggerWarn('sidebar-max-tools-active');
                return;
            }
            sidebarConfig.hidden = toggleInput.checked
                ? sidebarConfig.hidden.filter(hiddenId => hiddenId !== id)
                : [...sidebarConfig.hidden, id];
            item.classList.toggle('hidden-tool', !toggleInput.checked);
            saveSidebarConfig(sidebarConfig);
            applySidebarConfig();

            if (!toggleInput.checked && currentPanelId === id) {
                const fallback = sidebarConfig.order.find(fid => !sidebarConfig.hidden.includes(fid));
                if (fallback) activatePanel(fallback);
            }
        });

        item.append(handle, icon, label, toggleLabel);
        list.appendChild(item);
    });
}

const sidebarEditorOpenButton = document.getElementById('s-open-sidebar-editor');
const sidebarEditorBackButton = document.getElementById('editor-back');

sidebarEditorOpenButton?.addEventListener('click', () => {
    renderEditorList();
    activatePanel('sidebar-editor');
    sidebarEditorBackButton?.focus({ focusVisible: true });
});

sidebarEditorBackButton?.addEventListener('click', () => {
    activatePanel('settings');
    sidebarEditorOpenButton?.focus({ focusVisible: true });
});

document.getElementById('editor-reset')?.addEventListener('click', () => {
    sidebarConfig = { order: [...EDITABLE_SIDEBAR_IDS], hidden: [...DEFAULT_HIDDEN_SIDEBAR_IDS] };
    saveSidebarConfig(sidebarConfig);
    applySidebarConfig();
    renderEditorList();
});

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
        const themeRow = document.getElementById('s-app-theme')?.closest('.settings-row');
        const themeDefault = themeRow?.getAttribute('default')?.toLowerCase();
        document.body.setAttribute('theme', saved.appTheme || themeDefault || 'dark');
        const contrastRow = document.getElementById('s-high-contrast')?.closest('.settings-row');
        const contrastDefault = contrastRow?.getAttribute('default')?.toLowerCase() === 'enabled';
        if (saved.highContrast ?? contrastDefault) document.body.setAttribute('contrast', '');
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