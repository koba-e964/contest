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
const ll mod = 1e9 + 7;

bool lt(char a, char b) {
  cout << "? " << a << " " << b << endl;
  char ans;
  cin >> ans;
  return ans == '<';
}

string merge_sort(const string &s) {
  int n = s.length();
  if (n <= 1) return s;
  string fst = merge_sort(s.substr(0, n / 2));
  string snd = merge_sort(s.substr(n / 2));
  string ans;
  int p1 = 0, p2 = 0;
  while (p1 < (int) fst.size() && p2 < (int) snd.size()) {
    if (lt(fst[p1], snd[p2])) {
      ans += fst[p1];
      p1++;
    } else {
      ans += snd[p2];
      p2++;
    }
  }
  while (p1 < (int) fst.size()) {
    ans += fst[p1];
    p1++;
  }
  while (p2 < (int) snd.size()) {
    ans += snd[p2];
    p2++;
  }
  return ans;
}

string brute_force(string s) {
  int n = s.length();
  vector<string> all;
  do {
    all.push_back(s);
  } while (next_permutation(s.begin(), s.end()));
  while (all.size() > 1) {
    pair<int, PI> ma(-1e8, PI(-1, -1));
    map<PI, PI> tbl;
    REP(i, 0, n) {
      REP(j, 0, i) {
	int lt = 0, gt = 0;
	REP(k, 0, all.size()) {
	  int fst = all[k].find('A' + i), snd = all[k].find('A' + j);
	  if (fst < snd) lt++;
	  else gt++;
	}
	int sc = -abs(lt - gt);
	ma = max(ma, make_pair(sc, PI(i, j)));
	tbl[PI(i, j)] = PI(lt, gt);
      }
    }
    int x = ma.second.first, y = ma.second.second;
    int nlt = tbl[PI(x, y)].first, ngt = tbl[PI(x, y)].second;
    if (0)
      cerr << "asking " << x << " " << y << " divides " << all.size() << " = "
	   << nlt << " + " << ngt << endl;
    bool res = lt('A' + x, 'A' + y);
    vector<string> nxt;
    REP(i, 0, all.size()) {
      int fst = all[i].find('A' + x), snd = all[i].find('A' + y);
      if ((fst < snd) == res) nxt.push_back(all[i]);
    }
    all = nxt;
  }
  return all[0];
}

int main(void) {
  int n, q;
  cin >> n >> q;
  string a(n, '+');
  REP(i, 0, n) {
    a[i] = 'A' + i;
  }
  string res;
  if (n == 5) {
    res = brute_force(a);
  } else {
    res = merge_sort(a);
  }
  cout << "! " << res << endl;
  
}
