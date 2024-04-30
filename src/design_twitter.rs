use std::collections::{BinaryHeap, HashMap, HashSet};
struct Twitter {
    follows: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(i32, i32)>>,
    time: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            follows: HashMap::new(),
            tweets: HashMap::new(),
            time: 0,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        self.tweets
            .entry(user_id)
            .and_modify(|e| e.push((self.time, tweet_id)))
            .or_insert(vec![(self.time, tweet_id)]);
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut temp = Vec::new();
        let mut res = Vec::new();
        self.follows
            .entry(user_id)
            .and_modify(|e| {
                e.insert(user_id);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(user_id);
                set
            });
        let following = self.follows.get(&user_id).unwrap();
        for followee in following {
            if let Some(tweets) = self.tweets.get(followee) {
                let index = tweets.len() as i32 - 1;
                let (count, tweet_id) = self.tweets.get(followee).unwrap()[index as usize];
                temp.push((count, tweet_id, *followee, index - 1));
            }
        }
        let mut heap = BinaryHeap::from(temp);
        while !heap.is_empty() && res.len() < 10 {
            let (count, tweet_id, followee, index) = heap.pop().unwrap();
            res.push(tweet_id);
            if index >= 0 {
                let (count, tweet_id) = self.tweets.get(&followee).unwrap()[index as usize];
                heap.push((count, tweet_id, followee, index - 1));
            }
        }
        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows
            .entry(follower_id)
            .and_modify(|e| {
                e.insert(followee_id);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(followee_id);
                set
            });
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).and_modify(|e| {
            e.remove(&followee_id);
        });
    }
}
