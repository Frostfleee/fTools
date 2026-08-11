/*
   ╭─────────────────────────────╮
   │        FONT STYLER          │
   ╰─────────────────────────────╯
*/

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

/*
   ╭─────────────────────────────╮
   │      COLOR MATH HELPERS     │
   ╰─────────────────────────────╯
*/

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

/*
   ╭─────────────────────────────╮
   │        COLOR DETECTION      │
   ╰─────────────────────────────╯
*/

const colorParsers = [
    { // rgb / rgba (also matches % variants, comma or space separated)
        re: /^rgba?\(\s*([\d.]+%?)\s*[, ]\s*([\d.]+%?)\s*[, ]\s*([\d.]+%?)\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => ({ r: parsePercentOr255(m[1]), g: parsePercentOr255(m[2]), b: parsePercentOr255(m[3]), a: m[4] !== undefined ? parseAlpha(m[4]) : 1 })
    },
    { // hsl / hsla
        re: /^hsla?\(\s*([\d.]+)(?:deg)?\s*[, ]\s*([\d.]+)%\s*[, ]\s*([\d.]+)%\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = hslToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    { // hsv / hsb
        re: /^hs[vb]\(\s*([\d.]+)(?:deg)?\s*[, ]\s*([\d.]+)%\s*[, ]\s*([\d.]+)%\s*(?:[,/]\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = hsvToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    { // cmyk
        re: /^cmyk\(\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*[, ]\s*([\d.]+)%?\s*\)$/i,
        parse: (m) => cmykToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3]), parseFloat(m[4]))
    },
    { // oklch
        re: /^oklch\(\s*([\d.]+)%?\s+([\d.]+)\s+([\d.]+)(?:deg)?\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = oklchToRgb(parseFloat(m[1]) / 100, parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    { // oklab
        re: /^oklab\(\s*([\d.]+)%?\s+(-?[\d.]+)\s+(-?[\d.]+)\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = oklabToRgb(parseFloat(m[1]) / 100, parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    { // lch (CIE)
        re: /^lch\(\s*([\d.]+)%?\s+([\d.]+)\s+([\d.]+)(?:deg)?\s*(?:\/\s*([\d.]+%?)\s*)?\)$/i,
        parse: (m) => { const c = lchToRgb(parseFloat(m[1]), parseFloat(m[2]), parseFloat(m[3])); return { ...c, a: m[4] !== undefined ? parseAlpha(m[4]) : 1 }; }
    },
    { // Swift UIColor
        re: /UIColor\(\s*red:\s*([\d.]+),?\s*green:\s*([\d.]+),?\s*blue:\s*([\d.]+)(?:,?\s*alpha:\s*([\d.]+))?\s*\)/i,
        parse: (m) => ({ r: parseFloat(m[1]) * 255, g: parseFloat(m[2]) * 255, b: parseFloat(m[3]) * 255, a: m[4] !== undefined ? parseFloat(m[4]) : 1 })
    },
    { // Flutter Color(0xAARRGGBB) or bare 0xAARRGGBB (Android)
        re: /(?:Color\(\s*0x|0x)([0-9a-f]{8})\)?/i,
        parse: (m) => { const h = m[1]; return { r: parseInt(h.slice(2, 4), 16), g: parseInt(h.slice(4, 6), 16), b: parseInt(h.slice(6, 8), 16), a: parseInt(h.slice(0, 2), 16) / 255 }; }
    },
    { // Android Color.parseColor("#...")
        re: /Color\.parseColor\(\s*"(#?[0-9a-f]{3,8})"\s*\)/i,
        parse: (m) => hexToRgba(m[1])
    },
    { // hex with #
        re: /^#([0-9a-f]{3,8})$/i,
        parse: (m) => hexToRgba(m[1])
    },
    { // named color
        re: /^[a-z]+$/i,
        parse: (m, raw) => { const hex = namedColors[raw.toLowerCase()]; return hex ? hexToRgba(hex) : null; }
    },
    { // bare comma/space separated numbers
        re: /^([\d.]+%?)[\s,]+([\d.]+%?)[\s,]+([\d.]+%?)(?:[\s,]+([\d.]+%?))?$/,
        parse: (m) => ({ r: parsePercentOr255(m[1]), g: parsePercentOr255(m[2]), b: parsePercentOr255(m[3]), a: m[4] !== undefined ? parseAlpha(m[4]) : 1 })
    },
    { // bare hex without #
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

/*
   ╭─────────────────────────────╮
   │       COLOR FORMATTING      │
   ╰─────────────────────────────╯
*/

function formatColor({ r, g, b, a }, option) {
    switch (option) {
        case 'option1': // HEX
            return `#${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
        case 'option2': // HEX Shorthand (closest representable value per channel)
            return `#${toShorthandDigit(r)}${toShorthandDigit(g)}${toShorthandDigit(b)}`.toUpperCase();
        case 'option3': // HEX8
            return `#${toHex(r)}${toHex(g)}${toHex(b)}${toHex(Math.round(a * 255))}`.toUpperCase();
        case 'option4': // HEX8 Shorthand (closest representable value per channel, including alpha)
            return `#${toShorthandDigit(r)}${toShorthandDigit(g)}${toShorthandDigit(b)}${toShorthandDigit(a * 255)}`.toUpperCase();
        case 'option5': // RGB
            return `rgb(${r}, ${g}, ${b})`;
        case 'option6': { // RGB %
            const pr = Math.round((r / 255) * 100), pg = Math.round((g / 255) * 100), pb = Math.round((b / 255) * 100);
            return `rgb(${pr}%, ${pg}%, ${pb}%)`;
        }
        case 'option7': // RGBA
            return `rgba(${r}, ${g}, ${b}, ${trimNum(a)})`;
        case 'option8': { // RGBA %
            const pr = Math.round((r / 255) * 100), pg = Math.round((g / 255) * 100), pb = Math.round((b / 255) * 100);
            return `rgba(${pr}%, ${pg}%, ${pb}%, ${trimNum(a)})`;
        }
        case 'option9': { // HSL
            const { h, s, l } = rgbToHsl(r, g, b);
            return `hsl(${h}, ${s}%, ${l}%)`;
        }
        case 'option10': { // HSLA
            const { h, s, l } = rgbToHsl(r, g, b);
            return `hsla(${h}, ${s}%, ${l}%, ${trimNum(a)})`;
        }
        case 'option11': { // HSV
            const { h, s, v } = rgbToHsv(r, g, b);
            return `hsv(${h}, ${s}%, ${v}%)`;
        }
        case 'option12': { // CMYK
            const { c, m, y, k } = rgbToCmyk(r, g, b);
            return `cmyk(${c}%, ${m}%, ${y}%, ${k}%)`;
        }
        case 'option13': // CSS Named Color
            return nearestNamedColor(r, g, b);
        case 'option14': // Swift
            return `UIColor(red: ${(r / 255).toFixed(3)}, green: ${(g / 255).toFixed(3)}, blue: ${(b / 255).toFixed(3)}, alpha: ${a.toFixed(3)})`;
        case 'option15': // Android
            return `0x${toHex(Math.round(a * 255))}${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
        case 'option16': // Flutter
            return `Color(0x${toHex(Math.round(a * 255))}${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase() + ')';
        case 'option17': { // OKLCH
            const { L, C, H } = rgbToOklch(r, g, b);
            const Lp = (L * 100).toFixed(2);
            return a < 1 ? `oklch(${Lp}% ${C} ${H} / ${trimNum(a)})` : `oklch(${Lp}% ${C} ${H})`;
        }
        case 'option18': { // LCH (CIE)
            const { L, C, H } = rgbToLch(r, g, b);
            const Lp = L.toFixed(2);
            return a < 1 ? `lch(${Lp}% ${C} ${H} / ${trimNum(a)})` : `lch(${Lp}% ${C} ${H})`;
        }
        default:
            return `#${toHex(r)}${toHex(g)}${toHex(b)}`.toUpperCase();
    }
}

const ALPHA_AWARE_FORMATS = new Set([
    'option3', 'option4', // HEX8, HEX8 Shorthand
    'option7', 'option8', // RGBA, RGBA %
    'option10',           // HSLA
    'option14',           // Swift (UIColor alpha:)
    'option15', 'option16', // Android, Flutter (0xAARRGGBB)
    'option17', 'option18'  // OKLCH, LCH (alpha component when < 1)
]);

/*
   ╭─────────────────────────────╮
   │      COPY / TOAST LOGIC     │
   ╰─────────────────────────────╯
*/

const copyValidWarn = document.querySelector('#copy-valid.warn');
const copyInvalidWarn = document.querySelector('#copy-invalid.warn');
const fileSavedWarn = document.querySelector('#file-saved.warn');
const fileCorruptedWarn = document.querySelector('#file-corrupted.warn');
const fileInvalidWarn = document.querySelector('#file-invalid.warn');
const mediaConvertInvalidWarn = document.querySelector('#media-convert-invalid.warn');

function handleWarnAnimationEnd(e) {
    if (e.animationName === 'warn-out') {
        e.currentTarget.classList.remove('show', 'hide');
    }
}

copyValidWarn.addEventListener('animationend', handleWarnAnimationEnd);
copyInvalidWarn.addEventListener('animationend', handleWarnAnimationEnd);
fileSavedWarn.addEventListener('animationend', handleWarnAnimationEnd);
fileCorruptedWarn.addEventListener('animationend', handleWarnAnimationEnd);
fileInvalidWarn.addEventListener('animationend', handleWarnAnimationEnd);
mediaConvertInvalidWarn.addEventListener('animationend', handleWarnAnimationEnd);

function triggerWarn(warnElement) {
    if (appSettings.hideWarnNotifications) return;

    warnElement.classList.remove('show', 'hide');
    void warnElement.offsetWidth;
    warnElement.classList.add('show');

    clearTimeout(warnElement.warnTimeout);
    warnElement.warnTimeout = setTimeout(() => {
        warnElement.classList.remove('show');
        warnElement.classList.add('hide');
    }, 2800);
}

async function doCopyValid(text) {
    if (!text) return;
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

    triggerWarn(copyValidWarn);
}

function doCopyInvalid() {
    triggerWarn(copyInvalidWarn);
}

function doFileInvalid() {
    triggerWarn(fileInvalidWarn);
}

function doFileSaved() {
    triggerWarn(fileSavedWarn);
}

const fileCorruptedLabel = fileCorruptedWarn.querySelector('a');
const fileCorruptedDefaultText = fileCorruptedLabel.textContent;

function doFileCorrupted(message) {
    if (message) {
        const text = String(message);
        fileCorruptedLabel.textContent = text.length > 140 ? `${text.slice(0, 140)}…` : text;
    } else {
        fileCorruptedLabel.textContent = fileCorruptedDefaultText;
    }
    triggerWarn(fileCorruptedWarn);
}

function doMediaConvertInvalid() {
    triggerWarn(mediaConvertInvalidWarn);
}

/*
   ╭─────────────────────────────╮
   │      PERSISTENT SETTINGS    │
   ╰─────────────────────────────╯
*/

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

/*
   ╭─────────────────────────────╮
   │        PANEL WIRING         │
   ╰─────────────────────────────╯
*/

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

/*
   ╭─────────────────────────────╮
   │      COLOR PICKER SYNC      │
   ╰─────────────────────────────╯
   These reach into the picker iframe's DOM/CSS custom properties from
   the outside instead of touching the widget's own files. --hue/--light/
   --dark/--alpha are read reactively by --base-hsl/--clr-dark/--clr-final
   in the widget's own :root rule, so setting those four is enough; no
   need to also poke --base-hsl etc by hand.

   --sat is intentionally left alone at its native 100%. The box's
   rainbow-to-white background is a static image that only ever depicts
   fully saturated hues, it has no idea what --sat is. Forcing --sat down
   for greys/pastels made the handle sit on a spot the box renders as a
   vivid hue while the actual color underneath silently went grey, which
   is the "top grey, bottom white" mismatch.

   Now that colorpicker.html mixes real RGB (color-mix) instead of
   recomputing HSL, this box+darkness pair is exactly HSV in disguise:
   the box picks a hue tinted with white (HSV's S axis) and darkness
   mixes that toward black (HSV's V axis). That means any color,
   including true greys, is exactly reachable, no approximation needed,
   converting to HSV and mapping S -> light, V -> dark directly reproduces
   the input pixel for pixel. h stays HSV's own hue (same value HSL would
   give here since they share hue).
*/

function syncPickerFromColor(frame, rgba) {
    const idoc = frame?.contentDocument;
    const iroot = idoc?.documentElement;
    const handle = idoc?.getElementById('picker-handle');
    const darkness = idoc?.getElementById('darkness');
    const transparency = idoc?.getElementById('transparency');
    const box = idoc?.getElementById('picker-box');
    const preview = idoc?.querySelector('.color-preview');
    if (!iroot || !handle || !darkness || !transparency || !box || !preview) return;

    const { h, s, v } = rgbToHsv(rgba.r, rgba.g, rgba.b);
    const light = 100 - (s / 2);
    const dark = 1 - (v / 100);

    iroot.style.setProperty('--hue', h);
    iroot.style.setProperty('--sat', '100%');
    iroot.style.setProperty('--light', `${light}%`);
    iroot.style.setProperty('--dark', dark);
    iroot.style.setProperty('--alpha', rgba.a);

    const xPct = (h / 360) * 100;
    const yPct = ((light - 50) / 50) * 100;
    handle.style.left = `${xPct}%`;
    handle.style.top = `${yPct}%`;
    handle.style.borderColor = yPct > 50 ? '#000' : '#fff';

    darkness.value = Math.round(dark * 100);
    transparency.value = Math.round((1 - rgba.a) * 100);

    // Setting --hue/--light/--dark/--alpha above should be enough on its
    // own, the widget's own :root rule reactively recomputes --base-hsl/
    // --clr-dark/--clr-final from them. In practice, Chromium sometimes
    // just doesn't repaint elements driven by those custom properties
    // when they only changed via an ancestor's inline style, they keep
    // showing whatever color was there before, until something else
    // forces a style recalc (which is exactly why dragging always fixed
    // it manually). A plain forced reflow wasn't reliable enough to fix
    // this for every color, so instead of hoping the browser catches up,
    // the exact resulting colors are computed here in JS (mirroring the
    // widget's own color-mix math) and set directly as inline overrides,
    // guaranteed correct regardless of any repaint quirk. The moment the
    // user actually drags the box or either slider, those overrides get
    // cleared so the widget's own reactive CSS takes back over for live
    // interaction, exactly as it already did before any of this syncing
    // existed.
    const baseRgb = hslToRgb(h, 100, light);
    const darkRgb = {
        r: baseRgb.r * (1 - dark),
        g: baseRgb.g * (1 - dark),
        b: baseRgb.b * (1 - dark)
    };
    const baseCss = `rgb(${Math.round(baseRgb.r)}, ${Math.round(baseRgb.g)}, ${Math.round(baseRgb.b)})`;
    const darkCss = `rgb(${Math.round(darkRgb.r)}, ${Math.round(darkRgb.g)}, ${Math.round(darkRgb.b)})`;
    const finalCss = `rgba(${Math.round(darkRgb.r)}, ${Math.round(darkRgb.g)}, ${Math.round(darkRgb.b)}, ${rgba.a})`;

    preview.style.background = finalCss;
    darkness.style.background = `linear-gradient(${baseCss}, transparent)`;
    transparency.style.background = `linear-gradient(${darkCss}, transparent)`;

    if (!box.dataset.syncReleaseBound) {
        const release = () => {
            preview.style.background = '';
            darkness.style.background = '';
            transparency.style.background = '';
        };
        box.addEventListener('mousedown', release);
        darkness.addEventListener('input', release);
        transparency.addEventListener('input', release);
        box.dataset.syncReleaseBound = '1';
    }
}

function readPickerColor(frame) {
    const idoc = frame?.contentDocument;
    const preview = idoc?.querySelector('.color-preview');
    if (!preview) return null;
    // Reading getComputedStyle().backgroundColor as a string and
    // regex-parsing it broke once the color came from color-mix(): some
    // engines return that computed value as an unresolved "color-mix(...)"
    // string instead of a plain rgb()/rgba() one, which the regex silently
    // failed to match, so edits in the picker never made it back to the
    // input. Painting the color onto a 1x1 canvas and reading the actual
    // pixel back sidesteps string serialization entirely, canvas resolves
    // any valid CSS color, however it's expressed, down to real RGBA.
    const computed = idoc.defaultView.getComputedStyle(preview).backgroundColor;
    const canvas = idoc.createElement('canvas');
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
        if (!raw.trim()) return;
        const rgba = parseColor(raw);
        if (!rgba) { doCopyInvalid(); return; }
        doCopyValid(textSpan.textContent);
    });

    const pickerPanel = document.getElementById('colorpicker-panel');
    const pickerBack = document.getElementById('colorpicker-back');
    const pickerFrame = document.getElementById('colorpicker-frame');

    swatch.addEventListener('click', () => {
        const rgba = parseColor(inputBox.value);
        if (rgba) syncPickerFromColor(pickerFrame, rgba);
        panel.classList.remove('active');
        pickerPanel?.classList.add('active');
    });

    pickerBack?.addEventListener('click', () => {
        const rgba = readPickerColor(pickerFrame);
        if (rgba) {
            const opaque = Math.round(rgba.a * 255) >= 255;
            inputBox.value = formatColor(rgba, opaque ? 'option1' : 'option3');
            inputBox.dispatchEvent(new Event('input', { bubbles: true }));
        }
        pickerPanel?.classList.remove('active');
        panel.classList.add('active');
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

    function setProgress(percent) {
        if (loadingBarFill) loadingBarFill.style.width = `${percent}%`;
    }

    function showLoading() {
        setProgress(0);
        void loadingBarFill?.offsetWidth;
        panel.classList.remove('active');
        loadingPanel?.classList.add('show');
        windowEl?.classList.add('hidden');
    }

    function hideLoading() {
        loadingPanel?.classList.remove('show');
        panel.classList.add('active');
        windowEl?.classList.remove('hidden');
    }

    const isTauri = '__TAURI_INTERNALS__' in window;

    let originalFileName = '';
    let originalExtension = '';
    let originalFilePath = null; // real filesystem path, only ever set under Tauri

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

        const firstOption = matchingGroup.querySelector('option');
        if (firstOption) dropdown.value = firstOption.value;

        Object.entries(typeSvgs).forEach(([type, svg]) => {
            if (!svg) return;
            if (type === matchingGroup.id) svg.setAttribute('enabled', '');
            else svg.removeAttribute('enabled');
        });

        originalFileName = stripExtension(name);
        originalExtension = ext;
        originalFilePath = path;
        fileNameInput.value = originalFileName;
        uploadZone.setAttribute('data-filename', name);

        panel.removeAttribute('toupload');
    }

    function clearFile() {
        originalFileName = '';
        originalExtension = '';
        originalFilePath = null;

        if (fileInput) fileInput.value = '';
        fileNameInput.value = '';
        uploadZone.removeAttribute('data-filename');

        optgroups.forEach(group => { group.disabled = false; });
        Object.values(typeSvgs).forEach(svg => svg?.removeAttribute('enabled'));

        panel.setAttribute('toupload', '');
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

    unattachButton?.addEventListener('click', (e) => {
        e.preventDefault();
        e.stopPropagation();
        clearFile();
    }, { capture: true });

    resetButton.addEventListener('click', () => {
        fileNameInput.value = originalFileName;
        [metadataCheckbox, preserveCheckbox, overwriteCheckbox].forEach(cb => {
            cb.checked = false;
        });
        persistCheckboxState();
    });

    proceedButton.addEventListener('click', async () => {
        if (dropdown.value.toLowerCase() === originalExtension) {
            doMediaConvertInvalid();
            return;
        }
        if (!isTauri || !originalFilePath) {
            console.warn('Conversion needs the desktop app and a real file path.');
            return;
        }

        const mediaKind = typeSvgs.image?.hasAttribute('enabled') ? 'image'
            : typeSvgs.video?.hasAttribute('enabled') ? 'video'
            : typeSvgs.audio?.hasAttribute('enabled') ? 'audio'
            : null;

        if (mediaKind === 'video') {
            console.warn('Video conversion is not implemented yet (image and audio conversion work now).');
            return;
        }
        if (mediaKind !== 'image' && mediaKind !== 'audio') {
            return;
        }

        const { invoke } = window.__TAURI__.core;
        const { listen } = window.__TAURI__.event;

        showLoading();
        const unlisten = await listen('conversion-progress', (event) => {
            setProgress(event.payload);
        });

        try {
            const outputPath = mediaKind === 'image'
                ? await invoke('convert_image', {
                    sourcePath: originalFilePath,
                    outputName: fileNameInput.value || originalFileName,
                    targetExt: dropdown.value.toLowerCase(),
                    keepMetadata: metadataCheckbox.checked,
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

/*
   ╭─────────────────────────────╮
   │       SETTINGS PANEL        │
   ╰─────────────────────────────╯
*/

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
    if (appThemeInput) appThemeInput.value = appSettings.appTheme;
    if (highContrastInput) highContrastInput.checked = appSettings.highContrast;

    document.body.setAttribute('theme', appSettings.appTheme);
    document.body.toggleAttribute('contrast', appSettings.highContrast);

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
        document.body.setAttribute('theme', appThemeInput.value);
    });

    highContrastInput?.addEventListener('change', () => {
        appSettings.highContrast = highContrastInput.checked;
        saveAppSettings(appSettings);
        document.body.toggleAttribute('contrast', highContrastInput.checked);
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

/*
   ╭─────────────────────────────╮
   │        INPUT CLEAR          │
   ╰─────────────────────────────╯
*/

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

function getOpenDropdown() {
    const byOpenState = document.querySelector('select.dropdown:open');
    if (byOpenState) return byOpenState;
    return document.activeElement?.classList?.contains('dropdown') ? document.activeElement : null;
}

document.addEventListener('mousedown', (e) => {
    if (e.button !== 1) return;
    e.preventDefault();
    getOpenDropdown()?.blur();
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
    const openDropdown = getOpenDropdown();
    if (!openDropdown) return;
    if (e.target.closest('select.dropdown') === openDropdown) return;
    if (findScrollableAncestor(e.target)) {
        openDropdown.blur();
    }
}, { passive: true });

/*
   ╭─────────────────────────────╮
   │       TOOLBAR / PANELS      │
   ╰─────────────────────────────╯
*/

const toolbarButtons = document.querySelectorAll('.toolbar-button');
const panels = document.querySelectorAll('.window > div[id]');

function activatePanel(panelId) {
    const targetPanel = document.getElementById(panelId);
    if (!targetPanel) return;

    panels.forEach(panel => panel.classList.remove('active'));
    toolbarButtons.forEach(button => button.classList.remove('active'));

    targetPanel.classList.add('active');
    document.getElementById(`t-${panelId}`)?.classList.add('active');
}

toolbarButtons.forEach(button => {
    button.addEventListener('click', () => {
        const panelId = button.id.replace(/^t-/, '');
        activatePanel(panelId);
        if (appSettings.rememberLastTool && appSettings.lastTool !== panelId) {
            appSettings.lastTool = panelId;
            saveAppSettings(appSettings);
        }
    });
});

setupSettingsPanel().then(activatePanel);