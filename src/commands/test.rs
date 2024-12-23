use std::collections::HashSet;

use crate::program::Sfce;

impl Sfce {
    pub fn test_command(&mut self) -> anyhow::Result<()> {
        let sfinder = vec![
            "SITO", "ZITO", "SLTO", "ZLTO", "SJTO", "ZJTO", "ISTO", "LSTO", "JSTO", "ZSTO", "IZTO",
            "LZTO", "JZTO", "SZTO", "STIO", "ZTIO", "SLIO", "ZLIO", "ZSIT", "OSIT", "SJIO", "ZJIO",
            "TSIO", "SZIT", "LSIO", "JSIO", "OZIT", "ZSIO", "TZIO", "SOIT", "LZIO", "ZOIT", "JZIO",
            "SZIO", "STLO", "ZTLO", "SILO", "ZILO", "TSLO", "ISLO", "OZLT", "ZSLO", "TZLO", "IZLO",
            "ZOLT", "SZLO", "STJO", "ZTJO", "SIJO", "ZIJO", "OSJT", "TSJO", "ISJO", "ZSJO", "TZJO",
            "IZJO", "SOJT", "SZJO", "ITSO", "LTSO", "ZIST", "JTSO", "OIST", "ZTSO", "TISO", "LISO",
            "JISO", "OLST", "ZISO", "TLSO", "ILSO", "OJST", "ZLSO", "TJSO", "IJSO", "ZJSO", "IZST",
            "OZST", "IOST", "LOST", "TZSO", "JOST", "IZSO", "LZSO", "ZOST", "JZSO", "ITZO", "SIZT",
            "LTZO", "JTZO", "OIZT", "STZO", "TIZO", "LIZO", "JIZO", "OLZT", "SIZO", "TLZO", "ILZO",
            "OJZT", "SLZO", "ISZT", "TJZO", "IJZO", "OSZT", "SJZO", "TSZO", "ISZO", "LSZO", "JSZO",
            "IOZT", "LOZT", "JOZT", "SOZT", "SIOT", "ZIOT", "ZLOT", "SJOT", "ISOT", "JSOT", "ZSOT",
            "IZOT", "LZOT", "SZOT", "ZSTI", "OSTI", "SZTI", "OZTI", "SOTI", "ZOTI", "ZSLI", "OSLI",
            "SZLI", "OZLI", "SOLI", "ZOLI", "ZSJI", "OSJI", "SZJI", "OZJI", "SOJI", "ZOJI", "LTSI",
            "ZTSI", "OTSI", "TLSI", "ZLSI", "OLSI", "ZJSI", "OJSI", "TZSI", "LZSI", "JZSI", "OZSI",
            "TOSI", "LOSI", "JOSI", "ZOSI", "JTZI", "STZI", "OTZI", "SLZI", "OLZI", "TJZI", "SJZI",
            "OJZI", "TSZI", "LSZI", "JSZI", "OSZI", "TOZI", "LOZI", "JOZI", "SOZI", "STOI", "ZTOI",
            "SLOI", "ZLOI", "SJOI", "ZJOI", "TSOI", "LSOI", "JSOI", "ZSOI", "TZOI", "LZOI", "JZOI",
            "SZOI", "ZJTL", "JZTL", "OZTL", "ZOTL", "ZSIL", "OSIL", "SZIL", "OZIL", "SOIL", "ZOIL",
            "OTJL", "ZSJL", "OSJL", "SZJL", "OZJL", "TOJL", "SOJL", "ZOJL", "OTSL", "ZISL", "OISL",
            "ZJSL", "IZSL", "JZSL", "OZSL", "TOSL", "IOSL", "ZOSL", "OTZL", "SIZL", "OIZL", "SJZL",
            "OJZL", "ISZL", "JSZL", "OSZL", "TOZL", "IOZL", "JOZL", "SOZL", "ZTOL", "SIOL", "ZIOL",
            "ZJOL", "ISOL", "ZSOL", "TZOL", "IZOL", "JZOL", "SZOL", "SLTJ", "LSTJ", "OSTJ", "SOTJ",
            "ZSIJ", "OSIJ", "SZIJ", "OZIJ", "SOIJ", "ZOIJ", "OTLJ", "ZSLJ", "OSLJ", "SZLJ", "OZLJ",
            "TOLJ", "SOLJ", "ZOLJ", "OTSJ", "ZISJ", "OISJ", "ZLSJ", "OLSJ", "IZSJ", "LZSJ", "OZSJ",
            "TOSJ", "IOSJ", "LOSJ", "ZOSJ", "OTZJ", "SIZJ", "OIZJ", "SLZJ", "ISZJ", "LSZJ", "OSZJ",
            "TOZJ", "IOZJ", "SOZJ", "STOJ", "SIOJ", "ZIOJ", "SLOJ", "TSOJ", "ISOJ", "LSOJ", "ZSOJ",
            "IZOJ", "SZOJ", "LITS", "ZITS", "OITS", "ILTS", "OLTS", "OJTS", "IZTS", "OZTS", "IOTS",
            "LOTS", "JOTS", "ZOTS", "LTIS", "ZTIS", "OTIS", "TLIS", "ZLIS", "OLIS", "ZJIS", "OJIS",
            "TZIS", "LZIS", "JZIS", "OZIS", "TOIS", "LOIS", "JOIS", "ZOIS", "ITLS", "OTLS", "TILS",
            "ZILS", "OILS", "IZLS", "OZLS", "TOLS", "IOLS", "ZOLS", "OTJS", "ZIJS", "OIJS", "IZJS",
            "OZJS", "TOJS", "IOJS", "ZOJS", "ITZS", "OTZS", "TIZS", "LIZS", "JIZS", "OIZS", "ILZS",
            "OLZS", "IJZS", "OJZS", "TOZS", "IOZS", "LOZS", "JOZS", "ITOS", "LTOS", "JTOS", "ZTOS",
            "TIOS", "LIOS", "JIOS", "ZIOS", "TLOS", "ILOS", "ZLOS", "TJOS", "IJOS", "ZJOS", "TZOS",
            "IZOS", "LZOS", "JZOS", "JITZ", "SITZ", "OITZ", "OLTZ", "IJTZ", "OJTZ", "ISTZ", "OSTZ",
            "IOTZ", "LOTZ", "JOTZ", "SOTZ", "JTIZ", "STIZ", "OTIZ", "SLIZ", "OLIZ", "TJIZ", "SJIZ",
            "OJIZ", "TSIZ", "LSIZ", "JSIZ", "OSIZ", "TOIZ", "LOIZ", "JOIZ", "SOIZ", "OTLZ", "SILZ",
            "OILZ", "ISLZ", "OSLZ", "TOLZ", "IOLZ", "SOLZ", "ITJZ", "OTJZ", "TIJZ", "SIJZ", "OIJZ",
            "ISJZ", "OSJZ", "TOJZ", "IOJZ", "SOJZ", "ITSZ", "OTSZ", "TISZ", "LISZ", "JISZ", "OISZ",
            "ILSZ", "OLSZ", "IJSZ", "OJSZ", "TOSZ", "IOSZ", "LOSZ", "JOSZ", "ITOZ", "LTOZ", "JTOZ",
            "STOZ", "TIOZ", "LIOZ", "JIOZ", "SIOZ", "TLOZ", "ILOZ", "SLOZ", "TJOZ", "IJOZ", "SJOZ",
            "TSOZ", "ISOZ", "LSOZ", "JSOZ",
        ];
        let my = vec![
            "OZST", "IJOZ", "OZTI", "IJOS", "LZIO", "OZTL", "OZTS", "LZIS", "OSIJ", "OSIL", "OSIZ",
            "JSZI", "JSZO", "OSIT", "IJZO", "OSJI", "IJZS", "IJSO", "IJSZ", "JSZL", "JSTO", "LZOI",
            "LZOS", "IJTO", "LZOT", "LZSI", "OSJL", "OSJZ", "OSJT", "OSLI", "LZSJ", "LZSO", "IJTZ",
            "JSTZ", "JTIZ", "OSLJ", "LZTO", "OSLZ", "JTOZ", "OSZI", "JTOS", "OSZJ", "IOJZ", "OSZL",
            "IOJS", "OSZT", "OSTI", "OSTJ", "IOLZ", "OSTZ", "IOLS", "OTIZ", "IOZJ", "OTIS", "IOZL",
            "IOZS", "IOZT", "IOSJ", "IOSL", "IOSZ", "IOST", "IOTZ", "LZTS", "IOTS", "ILOZ", "LSIO",
            "LSIZ", "ILOS", "OTJL", "OTJZ", "OTJS", "JTZI", "JTZO", "ILZO", "ILZS", "OTLJ", "OTLZ",
            "OTLS", "OTZI", "OTZJ", "OTZL", "OTZS", "OTSI", "OTSJ", "OTSL", "JTSO", "OTSZ", "LSJO",
            "OIJZ", "OIJS", "LIOZ", "LIOS", "OILZ", "OILS", "LIZO", "LIZS", "OIZJ", "OIZL", "OIZS",
            "OIZT", "OISJ", "LSOI", "OISL", "LSOJ", "OISZ", "ILSO", "LSOZ", "ILSZ", "OIST", "LSZI",
            "LISO", "LISZ", "LSZJ", "LSZO", "OITZ", "OITS", "OJIZ", "OJIS", "LSTJ", "LSTO", "ILTZ",
            "OJLZ", "LITZ", "ILTS", "IZJO", "IZJS", "IZOJ", "LITS", "IZOL", "IZOS", "IZOT", "IZLO",
            "IZLS", "OJZI", "LTIS", "IZSJ", "IZSO", "IZSL", "LTOZ", "IZST", "LTOS", "IZTO", "OJZL",
            "OJZS", "IZTS", "OJZT", "ISJO", "OJSI", "OJSZ", "OJST", "LTZO", "OJTZ", "OJTS", "OLIZ",
            "OLIS", "LTSI", "LTSO", "ZIJO", "ZIJS", "ZIOJ", "ZIOL", "ZIOS", "ZIOT", "ZILO", "ZILS",
            "ISJZ", "ISOJ", "ISOL", "ISOZ", "ISOT", "OLJS", "LOIZ", "ZISJ", "LOIS", "ZISO", "ISLO",
            "ZISL", "ISLZ", "ZIST", "ISZJ", "ZITO", "ISZO", "ZITS", "ISZL", "ZJIO", "ISZT", "ZJIS",
            "ISTO", "ZJOI", "ZJOL", "ZJOS", "OLZI", "OLZS", "OLZT", "OLSI", "ZJLO", "ISTZ", "ZJLT",
            "ZJSI", "ZJSO", "OLSJ", "OLSZ", "OLST", "OLTZ", "OLTS", "OZIJ", "OZIL", "ZJSL", "OZIS",
            "OZIT", "OZJI", "LOJS", "LOZI", "ITJZ", "ZJTO", "LOZS", "LOZT", "LOSI", "ITOZ", "ITOS",
            "OZJL", "ZJTL", "OZJS", "ZOIJ", "OZLI", "ZOIL", "ZOIS", "ZOIT", "ZOJI", "LOSJ", "LOSZ",
            "LOST", "LOTZ", "LOTS", "ZOJL", "ZOJS", "JOTZ", "ZOLI", "JOTS", "OZLJ", "OZLS", "OZLT",
            "OZSI", "OZSJ", "OZSL", "JZOI", "JZOL", "JZOS", "ITLS", "JZLO", "ZOLJ", "ZOLS", "ZOLT",
            "ZOSI", "JZLT", "JZSI", "ZOSJ", "JZSO", "ZOSL", "ITZO", "ZOST", "ZOTI", "ZOTL", "ZOTS",
            "ZLIO", "ZLIS", "JZSL", "ITZS", "ITSO", "JZTO", "ZLOI", "JZTL", "ZLOS", "JSIO", "ZLOT",
            "JSIZ", "ZLSI", "ITSZ", "JIOZ", "JIOS", "ZLSJ", "ZLSO", "JIZO", "JIZS", "JISO", "JISZ",
            "ZLTO", "JZIO", "JZIS", "JSOZ", "JSOT", "JSOI", "JITO", "ZLTS", "ZSIJ", "ZSIO", "ZSIL",
            "ZSIT", "ZSJI", "ZSJO", "JITZ", "ZSJL", "SJZI", "TIZS", "SJZO", "TISO", "ZSOI", "ZSOJ",
            "ZSOL", "ZSOT", "JOIZ", "ZSLI", "JOIS", "SJZL", "ZSLJ", "ZSLO", "ZSTI", "ZSTO", "SJTO",
            "TISZ", "ZTIO", "TJIZ", "ZTIS", "JOLZ", "SJTZ", "SOIJ", "SOIL", "TJOZ", "TJOS", "SOIZ",
            "SOIT", "SOJI", "ZTJO", "ZTOI", "ZTOL", "ZTOS", "JOZI", "ZTLO", "SOJL", "SOJZ", "SOJT",
            "TJZI", "SOLI", "ZTSI", "TJZO", "ZTSO", "JOZL", "JOZS", "JOZT", "JOSI", "SIJO", "TJSO",
            "SIJZ", "JOSZ", "TOIZ", "JOST", "TOIS", "SOLJ", "SOLZ", "SOZI", "SOZJ", "SOZL", "SOZT",
            "SOTI", "SOTJ", "SIOJ", "TLOZ", "SIOL", "TLOS", "SOTZ", "SIOZ", "SIOT", "TOJL", "SLIO",
            "TOJZ", "SLIZ", "TOJS", "TLZO", "SILO", "SILZ", "SIZJ", "SIZO", "SIZL", "SIZT", "SITO",
            "TLSI", "SLJO", "TOLJ", "TOLZ", "SITZ", "TOLS", "SJIO", "TOZI", "SJIZ", "TOZJ", "TLSO",
            "TOZL", "TOZS", "TOSI", "TOSJ", "TZIO", "TOSL", "TOSZ", "SLOI", "SLOJ", "SJOI", "SLOZ",
            "SLZI", "SJOZ", "SJOT", "SZIL", "SZIT", "SZJI", "SZJO", "TZIS", "SLZJ", "SLZO", "SZJL",
            "TZJO", "TLIS", "SZOI", "SZOJ", "TZOI", "SZOL", "SZOT", "STOI", "SZLI", "TZOL", "STOJ",
            "TZOS", "STOZ", "SLTJ", "SLTO", "SZIJ", "SZIO", "SZLJ", "SZLO", "TZLO", "STLO", "TSOI",
            "STZI", "TSOJ", "TSOZ", "TSLO", "TSZI", "STZO", "TZSI", "TZSO", "TSZO", "SZTI", "STIO",
            "SZTO", "STIZ", "TSIO", "TSIZ", "TIJZ", "TSJO", "STJO", "TIOZ", "TIOS", "TIZO", "TILS",
        ];

        let binding = my.iter().collect::<HashSet<_>>();
        let collect = sfinder
            .iter()
            .collect::<HashSet<_>>();
        let d = binding
            .difference(&collect);
        println!("{:?}", d.collect::<Vec<_>>());
        Ok(())
    }
}
