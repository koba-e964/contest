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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;


class MulticoreProcessing {
public:
  long long fastestTime(long long jobLength, int corePenalty, vector <int> speed, vector <int> cores) {
    int n = speed.size();
    ll mi = 4e18;
    REP(i, 0, n) {
      double x = sqrt((double)(jobLength) / (double)(corePenalty) / speed[i]);
      if (x > cores[i]) {
	x = cores[i];
      }
      if (x < 1) {
	x = 1;
      }
      ll xx = x;
      for (ll k = max(1LL, xx - 10); k <= min((ll)cores[i], xx + 10); ++k) {
	ll tmp = (jobLength + speed[i] * k - 1) / (speed[i] * k);
	tmp += corePenalty * (k - 1);
	mi = min(mi, tmp);
      }
    }
    return mi;
  }
};


int main(void) {
  int jl, cp, n;
  cin >> jl >> cp >> n;
  VI speed, cores;
  REP(i, 0, n) {
    int s, c;
    cin >> s >> c;
    speed.push_back(s);
    cores.push_back(c);
  }
  cout << MulticoreProcessing().fastestTime(jl, cp, speed, cores) << endl;
}
