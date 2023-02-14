use std::collections::HashMap;
use std::cmp::Ordering;


pub struct Record {
    win_team : String,
    lose_team : String,
    is_draw : bool,
}

impl Record {
    pub fn new(line: &str) -> Self {
        let parts : Vec<&str> = line.split(";").collect();
        assert!(parts.len() == 3);
        let result = parts[2];
        let dt = match result {
            "draw" => Self {
                is_draw : true,
                win_team : parts[0].to_string(),
                lose_team : parts[1].to_string(),
            },
            "win" => Self {
                is_draw : false,
                win_team : parts[0].to_string(),
                lose_team : parts[1].to_string(),
            },
            "loss" => Self {
                is_draw : false,
                win_team : parts[1].to_string(),
                lose_team : parts[0].to_string(),
            },
            _ => panic!("invalid result"),
        };
        dt
    }
}

pub struct Team {
    team_name : String,
    match_played : i32,
    match_won : i32,
    match_lose : i32,
    match_draw : i32,
    points : i32,
}

impl Team {
    pub fn new(team_name : String) -> Self {
        Self {
            team_name : team_name,
            match_played : 0,
            match_won : 0,
            match_lose : 0,
            match_draw : 0,
            points : 0,
        }
    }

    pub fn process_result(&mut self, record : &Record) {
        if self.team_name != record.win_team && self.team_name != record.lose_team {
            return;
        }

        self.match_played += 1;
        
        if record.is_draw {
            self.points += 1;
            self.match_draw += 1;
        }else if self.team_name == record.win_team {
            self.points += 3;
            self.match_won += 1;
        }else if self.team_name == record.lose_team {
            self.match_lose += 1;
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:31}|  {} |  {} |  {} |  {} |  {}", self.team_name, self.match_played, self.match_won, self.match_draw, self.match_lose, self.points)
    }

}

fn compare_team(a : &Team, b : &Team) -> Ordering {
    let ret = b.points.cmp(&a.points);
    if ret == Ordering::Equal {
        return a.team_name.cmp(&b.team_name);
    }
    return ret;
}

pub fn tally(match_results: &str) -> String {
    let mut lines : Vec<&str> = match_results.split_terminator("\n").collect();
    let mut ret : String = "Team                           | MP |  W |  D |  L |  P".to_string();
    if lines.len() == 0 {
        return ret;
    }

    let mut team_map = HashMap::new();
    let records: Vec<Record> = lines.into_iter().map(Record::new).collect();

    for record in records.iter() {
        let team_name = &record.win_team;
        let mut team_win = team_map.entry(team_name).or_insert(Team::new(team_name.to_string()));
        team_win.process_result(record);

        let team_name_2 =  &record.lose_team;
        let mut team_lose = team_map.entry(team_name_2).or_insert(Team::new(team_name_2.to_string()));
        team_lose.process_result(record);
    }

    let mut team_vec = Vec::new();
    for (_, v) in team_map.into_iter() {
        team_vec.push(v);
    }

    team_vec.sort_by(compare_team);

    // team_map.into_iter().map(|(key, team)| {
    //     ret += "\n"; 
    //     ret += &(team.to_string());
    //     println!("{}", team.to_string());
    // });

    for v in team_vec {
        ret += "\n"; 
        ret += &(v.to_string());
    }

    ret
}
