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
string inf = "}";
namespace numero {
  typedef pair<string, int> ev;
  map<int, string> t_gen(int len);
  map<int, string> f_gen(int len);

  map<int, string> e_gen(int len) {
    map<int, string> ret;
    ret = t_gen(len);
    REP(i, 1, len - 1) {
      map<int, string> t1 = t_gen(i);
      map<int, string> t2 = e_gen(len - i - 1);
      for (auto &p1: t1) {
	for (auto &p2: t2) {
	  int nv = p1.first | p2.first;
	  if (ret.count(nv) == 0) {
	    ret[nv] = inf;
	  }
	  ret[nv] = min(ret[nv], p1.second + "|" + p2.second);
	}
      }
    }
    return ret;
  }

  map<int, string> t_gen(int len) {
    map<int, string> ret;
    ret = f_gen(len);
    REP(i, 1, len - 1) {
      map<int, string> t1 = f_gen(i);
      map<int, string> t2 = t_gen(len - i - 1);
      for (auto &p1: t1) {
	for (auto &p2: t2) {
	  int nv = p1.first & p2.first;
	  if (ret.count(nv) == 0) {
	    ret[nv] = inf;
	  }
	  ret[nv] = min(ret[nv], p1.second + "&" + p2.second);
	}
      }
    }
    return ret;
  }

  map<int, string> f_gen(int len) {
    assert (len >= 1);
    map<int, string> ret;
    if (len == 1) {
      ret[0x0f] = "x";
      ret[0x33] = "y";
      ret[0x55] = "z";
      return ret;
    }
    // !F
    map<int,string> f = f_gen(len - 1);
    for (const auto &e: f) {
      int nv = 255 ^ e.first;
      if (ret.count(nv) == 0) {
	ret[nv] = inf;
      }
      ret[nv] = min(ret[nv], "!" + e.second);
    }
    if (len >= 3) {
      map<int, string> e = e_gen(len - 2);
      for (const auto &p: e) {
	int nv = p.first;
	if (ret.count(nv) == 0) {
	  ret[nv] = inf;
	}
	ret[nv] = min(ret[nv], "(" + p.second + ")");
      }
    }
    return ret;
  }
}

namespace brute {
  map<PI, string> memo_e, memo_t, memo_f;
  string t_gen(int len, int des);
  string f_gen(int len, int des);

  string e_gen(int len, int des) {
    if (memo_e.count(PI(len, des))) {
      return memo_e[PI(len, des)];
    }
    string ans = inf;
    ans = t_gen(len, des);
    REP(i, 1, len - 1) {
      REP(bits1, 0, des + 1) {
	REP(bits2, 0, des + 1) {
	  if ((bits1 | bits2) != des) { continue; }
	  string t1 = t_gen(i, bits1);
	  string t2 = e_gen(len - i - 1, bits2);
	  if (t2 >= inf) { continue; }
	  ans = min(ans, t1 + "|" + t2);
	}
      }
    }
    return memo_e[PI(len, des)] = ans;
  }
  string t_gen(int len, int des) {
    if (memo_t.count(PI(len, des))) {
      return memo_t[PI(len, des)];
    }
    string ans = inf;
    ans = f_gen(len, des);
    REP(i, 1, len - 1) {
      REP(bits1, 0, 256) {
	REP(bits2, 0, 256) {
	  if ((bits1 & bits2) != des) { continue; }
	  string t1 = f_gen(i, bits1);
	  string t2 = t_gen(len - i - 1, bits2);
	  if (t2 >= inf) { continue; }
	  ans = min(ans, t1 + "&" + t2);
	}
      }
    }
    return memo_t[PI(len, des)] = ans;
  }

  string f_gen(int len, int des) {
    assert (len >= 1);
    if (len == 1) {
      if (des == 0x0f) {
	return "x";
      }
      if (des == 0x33) {
	return "y";
      }
      if (des == 0x55) {
	return "z";
      }
      return inf;
    }
    if (memo_f.count(PI(len, des))) {
      return memo_f[PI(len, des)];
    }
    string &ret = memo_f[PI(len, des)];

    string ans = inf;
    // !F
    string f = f_gen(len - 1, des ^ 255);
    if (f < inf) {
      ans = min(ans, "!" + f);
      return ret = ans;
    }
    if (len >= 3) {
      string e = e_gen(len - 2, des);
      if (e < inf) {
	ans = min(ans, "(" + e + ")");
	return ret = ans;
      }
    }
    return ret = inf;
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  vector<string> ans(256, inf);
  int found = 0;
  REP(i, 1, 16 + 1) { // up to 16 letters. 238 of 256 will be found
    cerr << "found = " << found << "\n";
    if (found >= 256) {
      break;
    }
    map<int, string> res = numero::e_gen(i);
    set<int> hit;
    for (auto &ent: res) {
      int bits = ent.first;
      string expr = ent.second;
      if (ans[bits] == inf || ans[bits].size() == expr.size()) {
	ans[bits] = min(ans[bits], expr);
	hit.insert(bits);
      }
    }
    for (auto bits: hit) {
      cerr << "ans[" << bits << "]=" << ans[bits] << "\n";
    }
    found += hit.size();
  }
  int rem = 0;
  REP(i, 0, 256) {
    if (ans[i] >= inf) {
      rem += 1;
    }
  }
  cerr << "remaining: " << rem << endl;
  REP(bits, 0, 256) {
    if (ans[bits] < inf) { continue; }
    int len = 16;
    while (true) {
      cerr << "trying len=" << len << " " << bits << endl;
      string sol = brute::e_gen(len, bits);
      if (sol < inf) {
	ans[bits] = sol;
	break;
      }
      len++;
    }
    cerr << "ans[" << bits << "]=" << ans[bits] << "\n";
  }
  cout << "vector<string> ans(256);\n";
  REP(bits, 0, 256) {
    cout << "ans[" << bits << "] = \"" << ans[bits] << "\";\n";
  }
}
