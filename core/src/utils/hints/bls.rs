use std::str::FromStr;

use field::{common::Bls12381Curve, fp12::Fp12};
use num::BigUint;

const H3_MUL_27: &str = "63891623516802577228883357667568145378824786485331985080248744262680520450716732645897490860103474055075576766097396818293708876769612339086911467311119166092421631146422397272917121732998188990440103040538223708007801683015710566261449586566077390336923964891534130441715602826527891570113247692353064514063061282038919654273666893154320563783920249146051673728223268760337546606352437290953125784615046262773527056891979619281408957453456187981074929011966614138885491898435063749928803517897632700708008736689651292843977184845622556886079548893956508033957439533763536250110912219597312813898772893080855506160170615114684567883143704791403759759811324903498309368838261424544947812954895901892222679941949194200879810623423915055862407212113412455475021293242588796274519521960484543986846525053401986459815194308219522440634613222237681677031691579160107942671182143112156637122918907232131781354702854510211877938824226576080521441668394350165032390142103556650460320422686107563004017265327514841859792969406722794942610566320781633542739075844424873806910916320809851390050472045921452575607199563736648443278747725156316681204956448187385254634395057429130315008823162706254712205607568797956380498973169205270065485596654413747895254293568246918665810764810162973728560";
const S27: &str = "202915375769921495847005799924839717720091091916743866730082729497403971766621466944999169202180005603686860994065763362120103686755750102034490721164619000173286488625820416232799250689957039232807497348695408309972718725874648443323847621985032437985040160349469377724032747300981016537877017630834023060790710219600988133754867908999049663443848724592796611422126320415280704597737883132760787352298090227358923343865017330787704544378900917942734784344924811187936664275681251394310275262494794572232547348259015980562473157824989626408500806915117476086865657326142010499712324397805765817592483406656592062202127408527005521920196323750406385917380529274857919418513138509405929160827656845640641642218159748208388772492723327424144120770513760021281683555003692496050643471142335102943396654503920634342979253061295049970371587889048305997051336518388926673100316223031886310242339498171401194440145904568288474524241186369896395525886707977372824353731223664449540925074582490012786205821303866232748071808361910734994706971623639422363112213331489820609849134860913856526913189058523867083932482340258838507242644997785696744188898978051140456418351956598609562825713089332268326887654988064850611713563465136805675815130708595022935364102304686432751324309406174768808786913474477957075760";
const H3_MUL_P: &str = "11936198574701264461588576466167042218828887759808462748828395852788468927448321584999951129540000329628638882003868433065888452162102947178499454186154058833722734625048259778399955922938649366635735138158553429998395219169096967254343977763825437528531774138204081042590161605940059796345706919460824885928865307035352243162051053470532333143755807328988035966007430612663570858690463713691811020723417072197583726109706901811041443786994171643690281432054400658113921427981250082018251486029105563072502785191706822386027832813234683906382400406771616240403862195655412382336019082223868577505440200391564238953066318148647383642364489632376846230434148780873995259912537559376819362401626873272978920130479985188728751323101372201420242398265515295369510797353158382120626086537784417820199803206112978490763485474193826468845387522885194470414784501081701569005900954295993312367196441068905952614126229680487557324955363904111552677993335763374872020807719039085267113239681322941928600342429639190161651282844818278529100410095508201315477189019499401212344066756524344501583128768148462769643087196485814029837802646928570396716994057532420026848138350388153503695630181725427548640450293415579447747856674419812098577360629917354290315535429687437220666135847422045224046289027910468063280";
const SP: &str = "152186531827441121887029117263540970990870634428323681863640513414312200632750787549379391358599357887373409380552196800897732675236184710367359954117064592882379073894674177899388927771535068237742209726327061482213986351087603657715324652128099621996042608777826405246828003944945043804566529968306847616985067322821659559040124856250368349090488401587773029782814747232184186051865169815135522364400024261934557113026958727612265270975619566770423559299067391390426801790426677827368414555160958811116877275580307338219299555499010311274263860320938733645318119352771966431283061063181089259030776783314591624815450358980833461055707537677101582990344944311460210668324057351596066575310694340467840893134589141764408876034596639851465863095202650989735218422144808606912288750391267682659617477599065046423535732158565787101662446314236771787806130533853852829187228600028362175533731905381666357972412731404886683153033746847150404364937926165698521638678095369175073260198773077416546885488830291551311165446974975130186580033423471866631851120174194633055796269919141638889821306136137618407117430423136391451976628180665290570454972781459364139013901646565273429192462457483343753915986328901589478568661697985880822986319508368635814453232191137428671694740209304602125084928572807439299280";
const LAM_INV_MOD_H3: &str = "1831641335620623066030493719814750505730353469232607408984136636587602798828541096315617787337660580896596326998534644457004429745922908233143891672764988414950129738729858091158205555565902620547217447631024516567208419316911441452506166124582175761417927641295939217754840037829611255709087158279242957940625143296408819749143295856831426628369017685600998542951914342403397019136966636888082401920580506454124412301032882712464981559302655280563177865634616387141297622595244784621594534214052542425250197179149807006335538265574872503469097186776521644002111433399650909641885318866460936246236243529502790661531948114436384119347931262823673992912046778604767497085407555474313407712735307333317882930639512124999103536572343436126871997578016786488693723339409687453114134784227807901874533332546112824417573327256689835936839706774393384112352709458439683933327013619962098977697684860933918339286553214389828566701676847442687594385334748280300467909322727863382808795828794016038705718095323209413819963715797469895587732751744906760424173767179546312457734136109353976884272162867417651164377486191418315758965239169722140695318540646880198727657740603762882647173358897627586506579349681824692808439213054305286657596509971558805903962977863660242510377179301897268259";

const BLS_K: &str = "76329603384216526031706109802092473003";
const BLS_H: &str = "322277361516934140462891564586510139908379969514828494218366688025288661041104682794998680497580008899973249814104447692778988208376779573819485263026159588510513834876303014016798809919343532899164848730280942609956670917565618115867287399623286813270357901731510188149934363360381614501334086825442271920079363289954510565375378443704372994881406797882676971082200626541916413184642520269678897559532260949334760604962086348898118982248842634379637598665468817769075878555493752214492790122785850202957575200176084204422751485957336465472324810982833638490904279282696134323072515220044451592646885410572234451732790590013479358343841220074174848221722017083597872017638514103174122784843925578370430843522959600095676285723737049438346544753168912974976791528535276317256904336520179281145394686565050419250614107803233314658825463117900250701199181529205942363159325765991819433914303908860460720581408201373164047773794825411011922305820065611121544561808414055302212057471395719432072209245600258134364584636810093520285711072578721435517884103526483832733289802426157301542744476740008494780363354305116978805620671467071400711358839553375340724899735460480144599782014906586543813292157922220645089192130209334926661588737007768565838519456601560804957985667880395221049249803753582637708560";
const BLS_LAMBDA: &str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129030796414117214202539";

const HAS_P_ROOT_CONTRIBUTION_EXP: &str = "3350213195463216969296304997042669292182389401426638013913961723521442020913211055594515233512333582640141955632271857207347236982219074023598456614853803075771416111043561555675814043854684681957025724444598076206726208576979749720063612751061099858341191003027355222281729893851638591717755840474098413160594729731100868423046837064509082150526448989817984830848410969445181806602490097664780193296456963333141664716060397180447486132246044299377393212097755675850351664736512636489551975370601609979808227769733995395925271059245615584559209533787507375359655404150219842706758811982729923825441259077020702247239724537510749785310819105130922716637249746330977027499192763252736334592302058599452465367113251366570102176175759218972928205234821633549857268856452737545705806717939743844776647985157258853649834874885505811756051055063249334589204307198201072093837504644595036373462557610402283673125429468192386076583531806792418172794462725662552469622238600418073582386348748747613462852631040893431141678269497094355495365573937283274627868794535992768459675345619364992466113468993619785105419976443374380063618620030257216789154370853233253911441351477034211534540186188622290096882773962638536974524866862919578703905036430882701319407325715783595735468173982089384033805366359344295906862313252621814070196436714897617526302930206166344937791280";
const HAS_27TH_ROOT_CONTRIBUTION_EXP: &str = "5632965166756991857591024505212528691841475092894717388136704565087822507432450802494284669519874402873416418480665100806290775541364477400937703629689371035937315254090285150757070064881314668480612946032522579510604262621367365044564577097024963601284987204354198810065898237396157211099576594739780293645019378946989009736638818383317123015448648020593608843152606938394286081515809293205295541790685737904922963533983145456834828686949421869217177963835329495445182746836224497024183182463836824279769935701144633022611157440904611528003964473602868951604621455313117460415964200375338923707378420160146504436104931758593524220565813122306747975222404483686549696271186394688253251654107956964514394689748756615724455282941177430960962550825260973509969337913010144256277389486978235527937297972618688920725013726647580858649185922515177179632590009854893090270807353576630790160577408356866564022264708571495705545052501340964363619209632421204302863476412263986561062541120398002521332211893588521697974551856065756042435333877406149451337865124114855768312534039330063437407629993396733516887948809859717184462003050319454597821272348655810431735711680111437192678741017378747725012560203950847271698513731796082650308554246393453160696918791542735140080048595404990417769351154482867451237457971751608253181943643031395378726498154366814010780509352143790394959843760";

fn pow_str(x: &Fp12<Bls12381Curve>, n: &str) -> Fp12<Bls12381Curve> {
    x.pow_vartime_extended(BigUint::from_str(n).unwrap().to_u64_digits().as_ref())
}

fn is_pth_residue(x: &Fp12<Bls12381Curve>) -> bool {
    pow_str(x, H3_MUL_27) == Fp12::one()
}

fn get_pth_root_inverse(x: &Fp12<Bls12381Curve>) -> Fp12<Bls12381Curve> {
    if !is_pth_residue(&x) {
        return Fp12::<Bls12381Curve>::one();
    }
    pow_str(&x, SP)
}

fn get_order_of_3rd_primitive_root(x: &Fp12<Bls12381Curve>) -> Option<u8> {
    let y = pow_str(x, H3_MUL_P);

    if y == Fp12::<Bls12381Curve>::one() {
        return Some(0);
    }

    let y = y.pow_vartime(&[3u64, 0, 0, 0, 0, 0]);

    if y == Fp12::<Bls12381Curve>::one() {
        return Some(1);
    }

    let y = y.pow_vartime(&[3u64, 0, 0, 0, 0, 0]);

    if y == Fp12::<Bls12381Curve>::one() {
        return Some(2);
    }

    let y = y.pow_vartime(&[3u64, 0, 0, 0, 0, 0]);

    if y == Fp12::<Bls12381Curve>::one() {
        return Some(3);
    }

    None
}

fn get_27th_root_inverse(x: &Fp12<Bls12381Curve>) -> Fp12<Bls12381Curve> {
    let pw = get_order_of_3rd_primitive_root(x).unwrap();
    if pw == 0 {
        return Fp12::<Bls12381Curve>::one();
    }
    let ord = BigUint::from(3u64.pow(pw as u32));
    let wj = pow_str(x, H3_MUL_P);
    let v_inv = BigUint::from_str(H3_MUL_P).unwrap().modinv(&ord).unwrap();
    let s = (&ord - v_inv) % &ord;

    pow_str(&wj, &s.to_str_radix(10))
}

fn h3_ord_element_lambda_root(x: &Fp12<Bls12381Curve>) -> Fp12<Bls12381Curve> {
    pow_str(x, LAM_INV_MOD_H3)
}

pub(crate) fn get_root_and_scaling_factor(
    x: &Fp12<Bls12381Curve>,
) -> (Fp12<Bls12381Curve>, Fp12<Bls12381Curve>) {
    // assert_eq!(pow_str(x, BLS_H), Fp12::<Bls12381Curve>::one());

    let wp_shift = get_pth_root_inverse(x);
    let w27_shift = get_27th_root_inverse(&wp_shift);

    let w_full = wp_shift * w27_shift;
    let x_shifted = x * w_full;
    let root = h3_ord_element_lambda_root(&x_shifted);

    (root, w_full)
}