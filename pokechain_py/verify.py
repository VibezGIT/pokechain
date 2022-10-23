chain = ["blipbug","glastrier","regidrago","obstagoon","nickit","toxtricity","yamper","redgieleki","indeedee","eternatus","spectrier","runerigus","stonjourner","rolycoly","yungoos","snom","morpeko","orbeetle","eiscue","eldegoss","sirfetchd","dragapult","toxel","lunala","arctovish","hatterene","espurr","rookidee","elgyem","milcery","yveltal","lurantis","sinistea","arctozolt","thievul","lycanroc","calyrex","xurkitree","eelektross","sizzlipede","eelektrik","kubfu","urshifu","unfezant","thwackey","yamask","kartana","alcremie","escavalier","raboot","tapu fini","impidimp","pincurchin","naganadel","litten","necrozma","arrokuda","appletun","nihilego","oranguru","uxie","emolga","applin","noivern","noibat","tapu bulu","ursaring","grimmsnarl","litleo","oricorio","oshawott","tapu lele","excadrill","landorus","sandaconda","araquanid","drakloak","kommo-o","octillery","yanmega","avalugg","grapploct","tapu koko","omastar","rillaboom","mr. rime","emboar","rockruff","frosmoth","hattrem","morgrem","melmetal","larvesta","aurorus","silicobra","amaura","aromatisse","electivire","empoleon","numel","lampent","togedemaru","unown","nosepass","skwovet","turtonator","ribombee","electrike","exploud","dreepy","yanma","aegislash","hatenna","accelgor","rowlet","type: null","litwick","komala","axew","wooloo","omanyte","entei","inteleon","ninjask","klefki","incineroar","reshiram","meltan","nincada","alomomola","amoonguss","sobble","elekid","duraludon","nuzleaf","falinks","stakataka","archeops","solgaleo","onix","xerneas","sandygast","tsareena","archen","natu","umbreon","noctowl","lilligant","toucannon","ninetales","steenee","espeon","nidoking","gossifleur","rufflet","trumbeak","keldeo","oddish","hoopa","arceus","stufful","liepard","dracovish","hawlucha","azelf","flapple","eevee","electabuzz","zarude","exeggutor","reuniclus","salazzle","exeggcute","electrode","ekans","salandit","torracat","trevenant","tyrantrum","marshadow","wimpod","dracozolt","tyrunt","thundurus","shiinotic","copperajah","heliolisk","kyurem","magearna","abomasnow","wishiwashi","igglybuff","fomantis","sylveon","nidoran","nidoqueen","nidorina","ambipom","minior","roggenrola","absol","lillipup","perrserker","regigigas","skrelp","polteageist","tornadus","slurpuff","florges","spewpa","anorith","heatmor","rotom","morelull","latios","scatterbug","greedent","trubbish","haxorus","stunfisk","klinklang","guzzlord","drednaw","whimsicott","tirtouga","altaria","azurill","latias","shelmet","throh","herdier","rhyperior","rampardos","sawsbuck","klang","golisopod","dubwool","luvdisc","cufant","timburr","rayquaza","aipom","malamar","registeel","lileep","pheromosa","azumarill","loudred","dottler","regirock","klink","karrablast","tranquill","lotad","drampa","ampharos","swanna","ariados","solosis","sigilyph","hippopotas","sawk","krokorok","kricketot","tepig","gumshoos","seismitoad","dewpider","relicanth","honchkrow","woobat","togekiss","swoobat","tangrowth","huntail","lugia","aerodactyl","larvitar","roselia","alakazam","meowstic","cursola","abra","arbok","kabutops","simipour","ralts","simisear","remoraid","durant","toxicroak","koffing","gourgeist","turtwig","gogoat","tropius","stoutland","deerling","genesect","trapinch","ho-oh","hitmontop","palossand","ducklett","torkoal","lapras","samurott","taillow","watchog","golurk","kingler","rapidash","houndoom","mandibuzz","zekrom","maractus","spiritomb","boltund","dewott","torchic","clobbopus","shellos","spheal","lickitung","golett","togetic","centiskorch","heracross","snorunt","tauros","slaking","gigalith","hoppip","phantump","pancham","mismagius","swellow","wormadam","mothim","metagross","smoochum","metang","garchomp","pawniard","drifblim","milotic","cosmoem","manectric","cosmog","glameow","wailord","deoxys","swinub","bisharp","palpitoad","dusclops","slowking","gorebyss","skiploom","medicham","marshtomp","probopass","seaking","grumpig","girafarig","gyarados","sandshrew","weezing","golem","mudkip","porygon-z","zweilous","spearow","wigglytuff","fletchling","gloom","mareep","prinplup","piplup","politoed","delibird","dewgong","golbat"]
print(len(chain))
used = {"blipbug": 1 }
for i in range(1, len(chain)):
    if chain[i] in used:
        print(f"{chain[i]} used more than once!")
        break;
    used[chain[i]] = 1
    prev = chain[i - 1][-1]
    cur = chain[i][0]
    if not prev == cur:
        print(f"Wrong at {chain[i-1]} -> {chain[i]}")
        break

print("done")
    