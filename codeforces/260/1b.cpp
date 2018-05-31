#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

const int DEBUG = 0;

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

struct node {
  int dp[2];
  node *ch[26];
  node() {
    dp[0] = dp[1] = 0;
    REP(i, 0, 26) ch[i] = NULL;
  }
  void add(const string &s, int dep) {
    if (dep >= (int) s.length()) return;
    int idx = s[dep] - 'a';
    if (ch[idx] == NULL) ch[idx] = new node();
    ch[idx]->add(s, dep + 1);
  }
  void calc(string debug) {
    REP(i, 0, 26) {
      if (ch[i] == NULL) continue;
      if (DEBUG) {
	ch[i]->calc(debug + (char)('a' + i));
      } else {
	ch[i]->calc("");
      }
    }
    REP(j, 0, 2) {
      int sum = 0;
      int nch = 0;
      REP(i, 0, 26) {
	if (ch[i] == NULL) continue;
	nch++;
	sum += 1 - ch[i]->dp[j];
      }
      if (nch == 0) {
	sum = j;
      }
      dp[j] = sum > 0 ? 1 : 0;
      if (DEBUG) {
	cerr << debug << "[" << j << "]" << " => " << dp[j] << endl;
      }
    }
  }
}root;


int main(void) {
  int n;
  ll k;
  cin >> n >> k;
  vector<string> s(n);
  REP(i, 0, n) cin >> s[i];
  // if 1kaime is a winning state, k = 1
  // otherwise, k % 2 matters.
  REP(i, 0, n) root.add(s[i], 0);
  root.calc("");
  int winning;
  if (root.dp[0] == root.dp[1]) {
    winning = root.dp[1];
  } else if (root.dp[0] == 0) {
    winning = 0;
  } else {
    winning = (k % 2);
  }
  cout << (winning ? "First" : "Second") << endl;
}
