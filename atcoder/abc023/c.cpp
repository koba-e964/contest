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
typedef pair<int, int> PI;
const double EPS=1e-9;

vector<PI> proc(const VI& tot) {
  int n = tot.size();
  map<int, int> freq;
  REP(i, 0, n) {
    if (freq.count(tot[i])) {
      freq[tot[i]]++;
    } else {
      freq[tot[i]] = 1;
    }
  }
  return vector<PI>(freq.begin(), freq.end());
}

int main(void){
  int r, c, k, n;
  cin >> r >> c >> k >> n;
  VI rtot(r);
  VI ctot(c);
  vector<PI> rc(n);
  REP(i, 0, n) {
    int rr, cc;
    cin >> rr >> cc;
    rr--, cc--;
    rc[i] = PI(rr, cc);
    rtot[rr]++;
    ctot[cc]++;
  }
  ll cnt = 0;
  vector<PI> rfreq = proc(rtot);
  vector<PI> cfreq = proc(ctot);
  REP(i, 0, rfreq.size()) {
    REP(j, 0, cfreq.size()) {
      if (k == rfreq[i].first + cfreq[j].first) {
	cnt += (ll) rfreq[i].second * (ll) cfreq[j].second;
      }
    }
  }
  REP(i, 0, n) {
    int rr = rc[i].first;
    int cc = rc[i].second;
    int tot = rtot[rr] + ctot[cc];
    if (tot == k) {
      cnt--;
    }
    if (tot == k + 1) {
      cnt++;
    }
  }
  cout << cnt << endl;
}
