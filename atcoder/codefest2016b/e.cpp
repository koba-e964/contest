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

const int N = 100010;
string s[N];

struct trie {
  trie *next[26];
  int chcnt;
  int len;
  int cnt;
  int occ;
  trie() {
    chcnt = 0;
    len = 1;
    cnt = 0;
    occ = 0;
    REP(i, 0, 26) {
      next[i] = NULL;
    }
  }
  void compact() {
    if (chcnt == 1 && !occ) {
      // compact
      int ch = -1;
      REP(i, 0, 26) {
	if (next[i]) {
	  ch = i;
	}
      }
      assert (ch >= 0);
      next[ch]->compact();
      trie *tmp = next[ch];
      chcnt = tmp->chcnt;
      len = tmp->len + 1;
      assert (cnt == tmp->cnt);
      occ = tmp->occ;
      REP(i, 0, 26) {
	next[i] = tmp->next[i];
      }
      return;
    }
    REP(i, 0, 26) {
      if (next[i]) {
	next[i]->compact();
      }
    }
  }
  void add(const string &s, int idx) {
    if (idx >= s.length()) {
      cnt++;
      occ = 1;
      return;
    }
    int ch = s[idx] - 'a';
    if (next[ch] == NULL) {
      next[ch] = new trie;
      chcnt++;
    }    
    next[ch]->add(s, idx + 1);
    cnt++;
  }
  void dump(int lv = 0) {
    if (occ) {
      REP(j, 0, lv) {
	cout << "-";
      }
      REP(j, 0, len - 1) {
	cout << "=";
      }
      cout << "*" << endl;
    }
    REP(i, 0, 26) {
      if (next[i]) {
	REP(j, 0, lv) {
	  cout << "-";
	}
	REP(j, 0, len - 1) {
	  cout << "=";
	}
	cout << (char)('a' + i) << endl;
	next[i]->dump(lv + len);
      }
    }
  }
  int calc(const VI &perm, const VI &invp, const string &s, int idx) {
    int sum = 0;
    if (idx + len - 1 >= s.length()) {
      assert (occ);
      return 0;
    }
    int pc = perm[s[idx + len - 1] - 'a'];
    REP(i, 0, pc) {
      if (next[invp[i]]) {
	sum += next[invp[i]]->cnt;
      }
    }
    sum += next[s[idx + len - 1] - 'a']->calc(perm, invp, s, idx + len);
    return sum + occ;
  }
};

int main(void){
  int n;
  cin >> n;
  trie t;
  REP(i, 0, n) {
    cin >> s[i];
    t.add(s[i], 0);
  }
  t.compact();
  int q;
  cin >> q;
  REP(loop_cnt, 0, q) {
    int k;
    cin >> k;
    k--;
    string perm;
    cin >> perm;
    assert (perm.length() == 26);
    VI p(26), invp(26);
    REP(i, 0, 26) {
      p[perm[i] - 'a'] = i;
      invp[i] = perm[i] - 'a';
    }
    cout << t.calc(p, invp, s[k], 0) + 1 << endl;
  }
}
