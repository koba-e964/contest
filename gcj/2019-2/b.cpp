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


VI read_vec() {
  int n;
  cin >> n;
  VI ans(n);
  REP(i, 0, n) cin >> ans[i];
  return ans;
}

int main(void) {
  // 20-B+1..B
  int B = 8;
  int MUL = 20 - B;
  int VOTE = 31;

  int t;
  cin >> t;
  VI ls(B);
  vector<set<int> > cont(B);
  vector<PI> srt;
  while (t--) {
    int i;
    while (cin >> i) {
      if (i == -1) break;
      int v = 1, p = 1;
      if (i <= 100 - B - VOTE) {
        v = 1 + ((i - 1) % MUL);
        p = 1;
      } else if (i <= 100 - VOTE) {
        v = 20 - (100 - i - VOTE);
        p = 0;
      } else {
        int idx = 100 - i;
        v = srt[idx == 0 ? 0 : (idx - 1) % (B - 1) + 1].second;
        if (i == 100) {
          p = 100;
        } else {
          p = 1;
        }
        // cerr << "putting into " << v << "!!!" << endl;
      }

      cout << v << " " << p << endl;
      if (i > 100 - B - VOTE && i <= 100 - VOTE) {
        VI ans = read_vec();
        ls[20 - v] = ans.size();
        cont[20 - v] = set<int>(ans.begin(), ans.end());
        if (i == 100 - VOTE) {
          srt.clear();
          REP(i, 0, B) srt.push_back(PI(ls[i], 20 - i));
          sort(srt.begin(), srt.end());
          if (0) {
            REP(i, 0, srt.size()) {
              cerr << " " << srt[i].first << "," << srt[i].second;
            }
            cerr << endl;
          }
        }
      }
      if (i == 100) break;
    }
  }
}
