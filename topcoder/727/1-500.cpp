#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

class Subrectangle {
public:
  int minMissed(string s) {
    int cc = 0;
    REP(i, 0, s.length()) {
      cc += s[i] == '#';
    }
    if (cc == 0) return 1;
    if (s == "") {
      return 1;
    }
    int len = s.length();
    VI conseq;
    char cur = '.';
    int cnt = 0;
    REP(i, 0, len) {
      if (cur == s[i]) {
	cnt += 1;
      } else {
	conseq.push_back(cnt);
	cur = s[i];
	cnt = 1;
      }
    }
    conseq.push_back(cnt);
    if (cur != '.') {
      conseq.push_back(0);
    }
    assert (conseq.size() % 2 == 1);
    int mi = 1e8;
    REP(m, 1, len + 1) {
      REP(wid, 1, m + 1) {
	int tot = 0;
	int blank = m - wid;
	bool ok = true;
	REP(i, 1, conseq.size() - 1) {
	  if (i % 2 == 1) {
	    int quo = (conseq[i] + wid - 1) / wid;
	    tot += (quo - 1) * m + wid;
	    tot -= conseq[i];
	  } else {
	    if (blank == 0) {
	      ok = false;
	      break;
	    }
	    int quo = (conseq[i] + blank - 1) / blank;
	    tot += (quo - 1) * m + blank;
	    tot -= conseq[i];
	  }
	}
	if (not ok) { continue; }
	int fst = conseq[0];
	int snd = conseq[conseq.size() - 1];
	int l = fst % m;
	if (fst % m > blank) {
	  tot += m - (fst % m);
	  l = 0;
	}
	int r = l + wid;
	int rest = snd - m + r;
	if (rest < 0) {
	  tot -= rest;
	  rest = 0;
	}
	int add = (rest + m - 1) / m * m - rest;
	rest += add;
	tot += add;
	if (0 && m == 5) {
	  DEBUGP(m);
	  DEBUGP(wid);
	  DEBUGP(tot);
	  DEBUGP(l);
	  DEBUGP(r);
	  DEBUGP(rest);
	}
	mi = min(tot, mi);
      }
    }
    return mi;
  }
};

int main(void) {
  string s;
  cin >> s;
  cout << Subrectangle().minMissed(s) << endl;
}
