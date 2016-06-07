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

const int DEBUG = 0;
using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

bool check(VI s, VI t, int tol) {
  int tot = 0;
  REP(i, 0, 26) tot += s[i];
  REP(i, 0, 26) tot -= min(s[i], t[i]);
  return tot <= tol;
}

int main(void){
  int n, k;
  string s;
  cin >> n >> k >> s;
  string t;
  int diff = 0;

  REP(i, 0, n) {

    int mi = i;
    vector<int> sfreq(26);
    REP(j, 0, n) sfreq[s[j] - 'a']++;
    vector<int> tfreq(sfreq.begin(), sfreq.end()); 
    REP(j, 0, i + 1) sfreq[s[j] - 'a']--;
    REP(j, 0, i) tfreq[t[j] - 'a']--;

    bool done = false;
    REP(c, 0, 26) {
      if (tfreq[c] == 0) continue;
      bool eq = s[i] == 'a' + c;
      tfreq[c]--;
      if (check(sfreq, tfreq, k - diff - !eq)) {
	t += 'a' + c;
	diff += !eq;
	done = true;
	if (DEBUG) {
	  cerr << "t:" << t << endl;
	  cerr << "diff:" << diff << endl;
	}
	break;
      }
      tfreq[c]++;
    }
    assert (done);
  }

  assert (diff <= k);
  cout << t << endl;
}
