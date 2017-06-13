// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentance.

use rand::{Rng, ThreadRng};

#[derive(Debug, Clone)]
pub struct Verb(String);
impl Verb {
	pub fn has_noun(&self) -> bool { self.0.contains("(noun)") }
	pub fn gen(&self, he_she_it: bool, noun: Option<&str>) -> String {
		// Consider making custom replace function for efficiency

		let mut string = if he_she_it {
			self.0
				.replace("[is/are]", "is")
				.replace("[has/have]", "has")
				.replace("[was/were]", "was")
				.replace("(s)", "s")
				.replace("(es)", "es")
		} else {
			self.0
				.replace("[is/are]", "are")
				.replace("[has/have]", "have")
				.replace("[was/were]", "were")
				.replace("(s)", "")
				.replace("(es)", "")
		};

		if let Some(noun) = noun {
			string = string.replace("(noun)", noun);
		}
		string
	}
}

#[derive(Debug, Clone)]
pub enum Word {
	Noun(String, bool),
	Ending(String),
	Verb(Verb),
	And
}

pub fn gen_noun(rand: &mut ThreadRng) -> Word {
	let (word, he_she_it): (&str, bool) = NOUNS[rand.gen::<usize>() % NOUNS.len()];
	Word::Noun(word.to_string(), he_she_it)
}
pub fn gen_ending(rand: &mut ThreadRng) -> Word {
	let word: &str = ENDINGS[rand.gen::<usize>() % ENDINGS.len()];
	Word::Ending(word.to_string())
}
pub fn gen_verb(rand: &mut ThreadRng) -> Word {
	let word: &str = VERBS[rand.gen::<usize>() % VERBS.len()];
	Word::Verb(Verb(word.to_string()))
}


const NOUNS: &'static [(&str, bool)] = &[
	("a bucket of vomit", true),
	("a cheap suit", true),
	("a cheese shop", true),
	("a cider-drinking chav", true),
	("a cold sausage without mustard", true),
	("a cream-faced loon", true),
	("a dead parrot", true),
	("a demon's asshole", true),
	("a duck hunter", true),
	("a frightened schoolboy", true),
	("a grunting sow", true),
	("a half-eaten crumpet", true),
	("a hamster", true),
	("a horny little bunny", true),
	("a lazy old codger", true),
	("a lumberjack", true),
	("a muppet", true),
	("a piece of battered cod", true),
	("a platypus", true),
	("a public loo", true),
	("a red-arsed baboon", true),
	("a ruttish fishmonger", true),
	("a strange woman lying in a pond", true),
	("all of those animals", false),
	("an alien hamburger", true),
	("an English pig dog", true),
	("angry mob", true),
	("an ordinary pigsty", true),
	("Dagon", true),
	("defenseless young men", false),
	("elderberries", false),
	("fish 'n' chips", true),
	("homeless man's socks", true),
	("Louisiana swamp-priests", true),
	("Satan", true),
	("some bloody minger", true),
	("some dirty rag", true),
	("some dog", true),
	("someone insane", true),
	("spam", true),
	("Stalin's jockstrap", true),
	("that Thing on your doorstep", true),
	("the African swallow", true),
	("the afterlife", true),
	("the communists", false),
	("the cozzers", false),
	("the Deep Ones", false),
	("the Hell's Grannies gang", false),
	("the meaning of life", true),
	("the Nazis", false),
	("the Queen", true),
	("the Royal Family", true),
	("the thing you had for lunch", true),
	("this bloody pet shop", true),
	("this conversation", true),
	("this lake", true),
	("this liver", true),
	("this parrot", true),
	("this place", true),
	("this poor man", true),
	("this train", true),
	("vodka", true),
	("you", false),
	("your beloved auntie", true),
	("your country", true),
	("your cousin's car", true),
	("your face", true),
	("your father", true),
	("your favourite Bond actor", true),
	("your hat", true),
	("your house", true),
	("your hovercraft", true),
	("your husband", true),
	("your kettle", true),
	("your liver", true),
	("your math teacher", true),
	("your mother", true),
	("your pimply arse", true),
	("your seat", true),
	("your sense of style", true),
	("your sister", true),
	("your sins", false),
	("your son", true),
	("your wife", true),
];
const ENDINGS: &'static [&str] = &[
	"mate!",
	"honey!",
	"mister!",
	"ma'am!",
	"sir!",
	"lady!",
	"and everybody knows that!",
	"and I damn you!",
	"and I don't love you!",
	"and I have proof!",
	"and I'm Serious!",
	"and it's scientifically proven!",
	"and now let me finish the rites!",
	"and piss off!",
	"and stop being a berk!",
	"and that's a load of bollocks!",
	"and that's racist!",
	"and the Dead One still dreams in R'lyeh!",
	"and you can't deny it!",
	"and you know it's true!",
	"because I don't exist!",
	"because you are a spoiled brat!",
	"cockwomble!",
	"I bet!",
	"innit?!",
	"и я знаю, что ты шпион!",
	"like a minging peasant!",
	"now put a sock in it!",
	"nudge nudge!",
	"oh God, who writes this stuff?!",
	"or is it about the way I talk?!",
	"pardon my French!",
	"says the fortune cookie!",
	"so go back to your seat!",
	"so, you know, well, innit, eh?!",
	"товарищ! (tovarishch!) [friend!]",
	"which is an ancient Chinese secret!",
	"which makes me stiff!",
	"which might cause death!",
	"which will make you die or go insane!",
	"yeah baby!",
	"you son of a washerwoman!",
	"you daughter of a washerwoman!",
	"you cheeky bastard!",
	"you cheeky bint!",
	"you commoner!",
	"you cross-eyed, inbred muckspout!",
	"you dickbag!",
	"you ginger!",
	"you inbred twit!",
	"you lying git!",
	"you numpty!",
	"you pillock!",
	"you pitiful mortal!",
	"you posh fopdoodle!",
	"you tottering fool-born hedge-pig!",
	"young man!",
];
const VERBS: &'static [&str] = &[
	"[is/are] (noun)",
	"[is/are] afraid of my minigun",
	"[is/are] an ill-nurtured whey face",
	"[is/are] an old bugger",
	"[is/are] an organ in your abdomen",
	"[is/are] deceased",
	"[is/are] dull and ugly",
	"[is/are] full of eels",
	"[is/are] getting fat",
	"[is/are] interested in photography",
	"[is/are] minging",
	"[is/are] no more",
	"[is/are] not a part of Europe",
	"[is/are] not interesting",
	"[is/are] not migratory",
	"[is/are] not my creation",
	"[is/are] not Serious",
	"[is/are] old",
	"[is/are] racist",
	"[is/are] rather plain",
	"[is/are] rude",
	"[is/are] scratched",
	"[is/are] silly",
	"[is/are] stone dead",
	"[is/are] very naughty",
	"[is/are] vile",
	"[is/are] where you can sit",
	"[is/are] worthless",
	"[was/were] (noun)",
	"[was/were] born in (noun)",
	"[was/were] burnt like a witch",
	"[was/were] defeated by (noun)",
	"[was/were] in jail",
	"[was/were] owned by (noun)",
	"[was/were] teabagged by (noun)",
	"[has/have] a steaming romp with (noun)",
	"[has/have] an unsightly face",
	"[has/have] bad breath",
	"[has/have] bum cancer",
	"[has/have] no life experience",
	"[has/have] not finished puberty",
	"[has/have] not so much brain as ear wax",
	"[has/have] only local multiplayer",
	"[has/have] tiny feet",
	"[has/have] worse hair than (noun)",
	"act(s) like (noun)",
	"admire(s) pictures of (noun)",
	"ate Yul Brynner with (noun)",
	"belong(s) in one of those cages",
	"bother(s) me",
	"bring(s) insults to a sword fight",
	"can be found in Pokémon GO",
	"can lick my lead",
	"can't dance",
	"can't exercise because of (noun)",
	"can't hold liquor",
	"can't tie a tie",
	"change(s) into (noun)",
	"dance(s) like (noun)",
	"died because of (noun)",
	"do(es)n't like (noun)",
	"do(es)n't own a colour telly",
	"donated organs for (noun)",
	"eat(s) only non-gluten lembas",
	"enjoyed Batman v Superman",
	"fart(s) in your general direction",
	"farted on (noun)",
	"give(s) a butcher's hook at (noun)",
	"go(es) around murdering people",
	"had too much of Fungi from Yuggoth",
	"hide(s) in this shrubbery",
	"like(s) to flash in the park",
	"look(s) like (noun)",
	"look(s) like a sad lemur",
	"made a poor deal with (noun)",
	"make(s) fun of the Pope",
	"make(s) me sick",
	"move(s) like a pregnant yak",
	"must be fun at parties",
	"need(s) a liver transplant",
	"never heard the call of Cthulhu",
	"never partied in Moscow",
	"never watched Star Wars",
	"play(s) hidden object games",
	"pose(s) nude for (noun)",
	"pre-ordered No Man's Sky",
	"probably murdered (noun)",
	"put(s) on granny's clothing",
	"read(s) Necronomicon with (noun)",
	"secretly adore(s) (noun)",
	"send(s) you warmest pots and dishes",
	"sit(s) next to (noun)",
	"smell(s) of (noun)",
	"stalked (noun)",
	"still use(s) Windows Vista",
	"suck(s) at Overwatch",
	"support(s) (noun)",
	"swallowed a chewing gum",
	"talk(s) to strangers",
	"taste(s) like (noun)",
	"tell(s) dirty jokes at funerals",
	"used to steal from orphans",
	"vomit(s) after drinking vodka",
	"walk(s) silly",
	"wants some Wang",
	"wanted to be (noun)",
	"wear(s) second-hand clothes",
	"went to the Mountain of Madness",
	"will always be alone",
	"will be chopped to bits",
	"will be damned for eternity",
	"will murder for a seat reservation",
	"will reincarnate as a pigeon",
	"will soon kick the bucket",
	"worked with (noun)",
];
