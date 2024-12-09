pub static INPUT: &str = r#"5976845435256788938547575289345049112119612651477393649449588160905575946466506682384157294138922255116599155731634536996124884739605710481770917784551029435350609228276086638469895373179440576090763098447378134837845037287427413366305228511517707448594835365270592189295241295979978660581843737823886751306741409480192545816266972840628276614277613696358226878912644515186334821832352898462572966189404129366079316988253460716430562221368147753065222693969464273275711094182460494089653380192436636565192222949110765955246998345739206821444532827680526511971084121868279273732834103422238379627935532296324040742815772481619331352843114557144034324766429789904412768644498089408853557797536997354616513957104924557621737729772734324919783174581896451397173351582295945889922024732841689789455595403685972280863872412355165069482387722291657176533475587625312068165216906330396999215990282111238099769059197153617363371578933011913850184317149343358053352986462017766246302130133396574983847327701675634751148683412252916779769837241471741344832527349638894230792338115354563830691086879347138680382683485363766721953912807439193751641656632788454827653611959279391477671197101975815469289030973343401474743775708696549839492326205953166593376720696184279221889054521150102623482850245128472214937112659142632551687630289337262120294122887662734862394650358650621273456833754225209341864697272597934833418420337532722513464617655740344117255797443990933192305214316115911566554030996123608474346798831239803621662952159639324581787276588175984898679355766747563884287836535382489378206716355139618291376032153887526936447018484138844159611855191318527411745481149139666137302450793670857541914587229549399356842113806028434679281773988555197020264743704769855341126743833332787363312352813936503483381248742692959458587925169954363860104111135433803696329889702191683133246463793245232813208216178296753789383369194756204094239325488142389153133748379255728762529682371698788851245471785296383336222753127714405414608943549281435457459758958675976067694636354834842462247996887624301189702290142840956270902020663020716558997535838667163492884921938370968997758710996287324316113952947676438994835480335375523364148954947870227337709068424862429326503457288664897081531263567624523857779036587850242769137944202377995822632619605175556622935878149755179567553281629863561869614176571896267837771964165232381782934228414336882874845244421817132732318663121991831523883423585311474985907674169363931920573474336640411685707641626250997073326555164119974815746655922062507417657363754450519356285663711284786051595927148778612143574237715184644064433457872716841021202091638068912940944634637274726167845587421187718481629283996995371256389719725651642848477288775382822133135486456327401786319557418476981712351559302080202911489743539131187218697557447335677881207221531336927466354082407897867238538218233584752380267299658824604926121487781335228699483075429392868053888826529139328754729634627195849762249447301590709953776054441893924849908662505156978027527434846882956667695159848560519465542388214628629851473666516221195282198567757470119416642165194945425186799540366037142984456066578792373484917828349772114318505221108795345480969039999065719461211640142678317529695233911246409081979227311879583231729091677239652486798644991096996466484492316133788736161672178666709480812685415958625597575041281563146469743752663027634054817383279877799937522232617064617642727965921030979162265044579240779835505232246816887215272337539835323243995064322788278565387853744547262018106330694813744132791826948888774432118565756938444518263414255683712732343567807759587291532392443291157325744713957058962365535157238174871384167454544473975920971979403185155726461212396291717082331478617696262575371968619032626999917787756876614347548043258574414634671439692354166267779621293125243743645383953945904024557281245782993835611496854913382260159972386073893524485874718517648652209826612255438475214368826018348911803763734175256811849334782913767383149546306841136684782842501775536879274325903896502334513883851157349251629428859553605918725337308683236368459654635831551437451324627581727112136345721435341611523511827675261825308180925156129968112690823946262889369851462474177981974556121116539766935636392658257032892076561119297573225524774082207869361393106157564469636390845475238177706215236915925191914277272439714511793914576472265914103278606719549169309321376962406971314781914321383923212318572590891597599126131630833881204125228246843021199414885728757611158535763762258822289746314292428433984365578120119315958424938753268361304714591936693360104192696442118733727992802436638759245816134484878931958977631320144321301278699983578522953971924716916589443416532988317395263588289956384795219495563246358794665851401054393260541550114792228241725512246299636576677238315986218655233988963318703529211185635315899187663189393285799850471122463093112194479870289394306845592046347254569724313489187799673130742497602684412379499579105879518120639852809847749537634550175524992195705210579496471985726053433920405251659955149878126644856488346787899524427695582752639459964821235252626887756628348889794689359055948178345816253757431652381130508472234319696593142180417541133575573089281098261554474019635620618012882560915280516387758188596256685191481081803593207992303822342488622065447890481691468681637333399199322886289385791675166327612297288312254791443377775851424477244193918451371468662013436480445099817147294261551365729424704319155378588625838451608373647281608357133421331259506259645953898579848526462567844351204814402749684142308143984963542785684382269050441784331947839766711656918142329957502140286182105998997030678075557062896418998720428769498989923595583636953371359527314363559665342331236797663561834845513967709878901316995290164732591940856852669490447633969766945659803355741198229027519752569077152855465275659952495828322298293360117673166681972038912187631997745418232887624494607536395060932685356820732267455544396268862459833472352598984628159748325448358635934468736161176571831254272282633346959084298859495993612594259273702087877810881221664083593242541712439162424160891010249158237761388148437723568184947399175276898353238495108250499887819883711061622294266442452294323739188346857776728640201468735016431966938365573684827367655720974893862678522649968416845376477957591443169882896387176681333499907344428458248726193083304879734870359226204212666054959864458772548133684036245375415783665522449927951261433534771764961392542621482863829265875493112810781232442719348854837526587356341775275540752146881748374890749999361112567695504894249975464422447157741243999358272897847135562912134534109932208139284943929855208152665790481371215660678628969567469289999546194899858543884392508193425651452457124955375376966421544559336485125011673125135124314316961734682246234678909340321978554537269477571195257714332478368990102346148929281622128129296583387159977981613735813696352713694559132476658388514358364555711373997013533781502446362731202445234015805256712267913223546074537763959366995822899311197631469083435371235992848954576967713198823253617360571159627678786690423288605974992258411058582880356931673918312495332655685088989187538328691961706128434068978314351158731845727229602432124590757481291354676867654278981825509343865288567285854696447399295756905794623762665553658925672699673568692348646952449457897457675055872046804038387210111419883642351158775599629834453193162415416173399051387684574376548786745841493577597665185088187924861026945794772846268610238448294022866886592929838296115487813144386429324560125898432095464871324782495561465625545418608966265618857650693986805612439273463631523039461528618020957334881377309864694184114259117818108142341639267282463545478388399654233670781718761644254450645624661215322938264890683554835663722030314080193187233594455761958074114189321913116012562682636683282282229455464948926678735330651027383417266511545061989143949481782141794245966364777673971821154652874639778162715259607857993925159867834693145830535536559628516725631177701869349111386773201945595252114837378728166048632198738478268495983328907854584278368616492678575238476095742216919239288452604549387232516530911460963940341649612323378445739452326125478312464843596720301651621846537490179161636718895666828894564326693036953133499987496930844335705233579525747450568372814685295682887045835460633062371257799274332071755678687032379743116133597467725815145842598646788027511144569278415040835068959984829767411824736930996044223428278656279820647668446064449833157898497781748458587746206591133114394195209396103287125373791354727236868757415418548659494316264956511667403464774028106511813916849965407392319779778055452892292841878336699673246894263297182013859982763223808468388873909530319577185815752345327411363137291697181916486125788987164130134448248367383796152622526935481935201011514155246467302991945620249733245083304625741991279960892741958624692121868066872191802367489920134256316443314714129153475280214311411756653637963967415637903088953398535115892439577743196195612458864653962242421712443871272475375460292826687320859491801429341545823487607716604184272783571528983623287294958819791583739340702145341618789823143757111456734063296898118049944516679932767764278321238079225563998357236134914156297991523712286144658896325783568997679682191399608955517121939583347191536349416015327462749741337461411625705568618486246428636913777592252091736188496726869699563661198138398050414677109476242227567561846534297269149022469468458578998052457277531357885170269798284623787284645038781635198014792115265840399852855913977487593471556797778010426168305743141278707915626771303874454585362841142035321849227268626866897691658148399474434648446735198080862579215470168842923958669532133829606655891492814582117047725454342679422982545060994794686894576662799353351323261616413845361659886875546755594081307951119461997769774678747469855042896250825647398890831684655826367976576836734922109198258915906119798747105442648113131968579446232149784487643688842076959874587187434328686679828663987190631970536118647788833091291142563348341680519543535674144391916051879923949226678675323318827724118251911726758390481460906027225171694597441696869791139347295050777681309133604387641984296491318546253022514637755754807145973588402339722434743621651573134630664744553661745586235942496635863344332844782140118165312718397620316820907271176274333041614388879396329925469953442867285410347119277459646472822860881144951623151449216192708636296387381729723385284979337931839089873813271731748879743234656689954039105190551887236753622584898621131487433862697274944137244536809320517615691298674268732073675111852819666035127772737813505791262138403639566035478134339277321182632375573535285332941029417779579176623788116362784636477733303760597496417191308145284023718594693674379831754528112325807392878753453620895623715387253887534681759795651385484174322564746154688884341511864046708332287829119836876985904018677523693450676651188870643054387528625728806797686440278345479544359336432970897381863748965382633570854653102087687228836635294418185784347166577235354865836554306730106696938533857478875055885859495818571699373137503670556393995343319839951354642526742465937393116458509826118255205422824594316655659268304753575860409215956967738876627461308031711382252775879039865552932995721923398396378373708462602292276031736161559531798277256663869823415616573262831337318712602721139132478427581360712923716881121316391752487355252312646242166133713213183841727888142867169782127093655210303922509313226768603050344433527437168224856461878744276865802946748957961339918740172833352186784348913631145734407998638049901797866653775648685131455168171623267740682354734593598260174437596595791288467311578960654168253874533029285547694242921582486885996073622727103042806816685964203237806067311410952961809017384815166274251264851850858131591945721287295891521292859290444786774684866513825134829459775888428095124048882396101011417860431093348947598178609984306841108154254042417278829414787718979346282578331755489948989826393699218154778771527169391194531298261192503975752313731267224542458594178778682587135312153964791567841931494038974651952627716845848079427188542316646662745877514082439395205465343221231399145594643383764388333375134864674954621580202644267972112983467236144238231269614745255618199893391021223838217278562351466989512243834916115538615273375333578326379850815660809149741739234014677515501579706484935098433321496959395039302956619950989956315284224238319946832488362758169988402253648435362445907810251510224093589771111714228618784184451933935730281080934019292331333727168516696762939733191867467272138156525533653055729548668242384186512711166266144182366059265724623872906095844418847965981198455078293383881557717277208640285687113442402679148631727552249089635783979886122318672984871453284564226626698683655588888377615799395991457923314242891635275826485712463790159844504172612037946850551785271097279726542924916214191444856695357558777689623730611866254435164643749114997958451010874369824715593536944521712693705465974147812742839289186561257819424925527354702989882423156788186358109219484991221389304950347911735015385124889949889468613244819973874690482566401855881056474832541796857565155194159631633299909112208870438762475537805223433965126541906275631424858791447924882334493317318092637962837521978086721078183836624716762251474855808497476946525413111395506455112737336950299580273555903679852381227262434930992159575015545578515267936017524242292168697763238355531483952679943153613944631867901993215271127898427029469728387985129980165494718774235821939660458693429985739892357417732453217118605074285176578081793248419939994935318426509481221122143279188775414097878310547450353946445569452788513181974410869592918647643640579081559217428821288439259991629047652743731853723138793387721864593480196662774769248195481595866450437610599941114925204254116071189983408536997166143571125660694082266522344096254519773966632049291827269174466088816948781358191313118993468125863337397424493167794573351324682493709778476496914337241611262052737972664614873928144285614426191231232763635692555165228778712820617188612198706430236015691386451646157699916633865758322385552969294232998884727325178838882738615296107967623710708258306354735440972256671042506513715681726186446474235462355545844469861271249364316041111322626935537928924187675623363841437756607186404361455387439331963512351688309224466569927991904992632848338079265562249760669011809666933542222093671549927880558261612845632713854995505245987722643016283833289694225975334717301191542766955621983217551529842656977053141438842846808054496693755361756588492750918858578848334842415269624066526863217467942247647657779184897191745177668941128961764448419088288144318046613950557133918965897440733991579017213926734059644856961358387143489495736348426570522789318929186989937854477886988995679471431716499577233835878490241583809036742576228961223528187563327570597580793128116862716985396826529258854422823765244631225515887019632314747778191692278252337932588550221920234853649416277411631353294192332647511175916713957792965551158565862569792371476655935532248449629418854043488238256410411155131897332378152670562242703767531293128593781359994824173728199022188849873098324529884324225261813884866645571859499288501196665432269620884396983631968011611422869284146497289834586174277589768940575553731592748850709121488640843373355420177948741520826820856915411190335676396195999776368979476214526749582211331182601957748763249397984541269368122149693630973752692377202090531413518434516997707547599851136752829753951639301196737380986636691659902590278920674850281837779287213944428161355080602890148839743439303732157789354616498234106411121149195861672328186630288793445469456262801826854486538799649415176047387976746770502137775495479765361994507217691912608462289475291676529971293144793689205071325222774082917913678996908487674568942963139225641281213774249951386461363735617974413766505622692666473733857260776312454537362182574828871313581878121441285184724534645218225142247138879436246162123791591717857297577426559884169592397141707346626449546864895389176065902047855481413793785777308711448648129540204526401190631726579642474671379115186565977142624884651065956315773729966767937854988332429316182541865462176839328530312712926923221686128512184141242149782265547323933177413512129736552529151423662452361623255621994288196344733150857999121328549661359161241830954776401856854265291026164928216825895329953994544631455488882211719614159221843836267853571689247965503731389194822126136577905728244593654764216033309118137342397879139018673581296496659013383647752788949662344022702627772329993474465388752576636364988988381357843116319040953221294476774855608345623314988389755623818521249777223882185073328655203427935937999858423412891521993571842663579034736245774174816943445447588189872092473285834473114627425271199620665998997860257758397070583052203423915275137475245292977464271731242111603833622831961154122988511726685181611581754058215688476167645513252292662345572739432978388947624812945016513710284938811115906920205820904013948773796258998786697644472998902344517399638717713296353643876682258159809339347561627644979246258235446111545227911764865666127635283313259938159110414031759894857459224730606048262055684770736617777273324749163540164847188096667926125822203892506066966081429271999782428333466295777348791841903119188480933256938033416533706936499978711594515172445876123617317929422435644887613630575241857153715759639459538119225253137060263022932055911076956027331528946598667438107279949476879462602073814559447646902239351946116856112973938032274438267542546746427247173521192016501134164050313837568275774368748376966848695330405421709619296971601821382766901449803841496394564152839169298240381394263859666513842088714956941061165842881654319824951839344753461069589117631255948684327668618858373439484847907484605714477287176061623499887467488322711725375344387912141977155590372820955067943228839174364837255844621750839856663975847837742838275716461870169028544026187342725468383351594056537074512228809051649591564519164847705281683610836427899247928661846623503712142073806422552939975229417421854278507435264988109027985088291434607860289125347513365066135781573179518310534127844065515648361285664218749321471524529779872855357540652365528567262890309814532485144640261388204733635923895832375314876045722142927426914488465782721738376458494331862310796198526484283765388339964876148267572844848375522189137747587440335075712496559661527124536669302028288678786544783883743599594932184054976973641122836627777176913697836627455210902311933198173833104778123323458236143024914098185542817468521637807634567882993983903695274246395152741689597442445312787212125462312511799751272329527195976512687924813386459325161595138232807133872668236987222665579583389679126240154165469895442568484618596861284228173063265142866773193886118984877788939276602134776040258152561850104015629839532291468677255034683456827735613299236133904051905044698260423934219353592371868428153619961289322116226676207086772132836691785080548545621151766570813359482525557712377228269765535272942083743515899212401995941816643569996355602866967249341021798124492028355550198353622823168545645859979797364699198692309247328840755089776310225796449635432590591135373439309964196623857988619780473752139353975125501137274266761484852780733178173130121412507451131650253378797981112759951825403222661148882728849768458047842069316976114041812816459485745827674629637398977039261815948212326628551468931149325054856242451969336971904895822533345786992453409049703164556810338565845861287277736934645921133740357738273341154368112340236571999413174435858260206"#;
