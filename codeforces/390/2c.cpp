#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

VI solve(int n, const VI &sender, const vector<VI> &mention) {
  int m = sender.size();
  if (0) {
    REP(i, 0, m) {
      cout << sender[i] << ":";
      REP(j, 0, mention[i].size()) {
	cout << " " << mention[i][j];
      }
      cout << endl;
    }
  }
  vector<VI> dp(m + 1, VI(n, 1));
  REP(i, 0, m) {
    if (sender[i] >= 0) {
      REP(j, 0, n) {
	dp[i + 1][j] = 0;
      }
      dp[i + 1][sender[i]] = 1;
    } else {
      REP(j, 0, mention[i].size()) {
	dp[i + 1][mention[i][j]] = 0;
      }
    }
    if (i >= 1) {
      REP(j, 0, n) {
	bool ok = false;
	REP(k, 0, n) {
	  if (j == k) continue;
	  if (dp[i][k] == 1) {
	    ok = true;
	    break;
	  }
	}
	dp[i + 1][j] &= ok ? 1 : 0;
      }
    }
  }
  bool ok = false;
  int v = -1;
  REP(i, 0, n) {
    if (dp[m][i]) {
      ok = true;
      v = i;
    }
  }
  if (not ok) {
    return VI();
  }
  VI ret(m, -1);
  ret[m - 1] = v;
  for (int i = m - 2; i >= 0; --i) {
    REP(j, 0, n) {
      if (j == ret[i + 1]) continue;
      if (dp[i + 1][j]) {
	ret[i] = j;
      }
    }
  }
  return ret;
}


int main(void){
  int tt;
  cin >> tt;
  while (tt--) {
    int n;
    cin >> n;
    vector<string> name(n);
    map<string, int> name_v;
    REP(i, 0, n) {
      cin >> name[i];
      name_v[name[i]] = i;
    }
    int m;
    cin >> m;
    cin.ignore();
    vector<string> lines(m);
    VI sender(m);
    vector<VI> mention(m);
    REP(i, 0, m) {
      getline(cin, lines[i]);
      stringstream is(lines[i]);
      string t;
      getline(is, t, ':');
      if (t == "?") {
	sender[i] = -1;
      } else {
	assert (name_v.count(t));
	sender[i] = name_v[t];
      }
      VI words;
      string rest;
      getline(is, rest);
      //http://stackoverflow.com/questions/7621727/split-a-string-into-words-by-multiple-delimiters
      size_t pos;
      size_t prev = 0;
      while ((pos = rest.find_first_of(" ,.?!", prev)) != string::npos) {
	if (pos > prev) {
	  string word = rest.substr(prev, pos - prev);
	  if (name_v.count(word)) {
	    mention[i].push_back(name_v[word]);
	  }
	}
	prev = pos + 1;
      }
      if (prev < rest.size()) {
	string word = rest.substr(prev);
	if (name_v.count(word)) {
	  mention[i].push_back(name_v[word]);
	}
      }
    }
    VI sol = solve(n, sender, mention);
    if (sol.size() == 0) {
      cout << "Impossible" << endl;
    } else {
      REP(j, 0, m) {
	if (sender[j] == -1) {
	  cout << name[sol[j]];
	  cout << lines[j].substr(1) << endl;
	} else {
	  cout << lines[j] << endl;
	}
      }
    }
  }
}
