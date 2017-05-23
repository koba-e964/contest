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

string calc(const VI &diff, int v) {
  string ret;
  REP(i, 0, diff.size()) {
    if (diff[i] >= 0) {
      ret += 'a' + v * diff[i];
    } else {
      ret += 'z' + v * diff[i];
    }
  }
  return ret;
}

ll hasher(const string &s, ll a, ll b) {
  ll h = 0;
  REP(i, 0, s.length()) {      
    int c = s[i] - 'a' + 1;
    h = (a * h + c) % b;
  }
  return h;
}

int main(void){
  ll a, b;
  cin >> a >> b;
  // Find a colliding assignment {-2, ..., +2} of length 25
  ll num_trial = 0;
  const int B = 25;
  map<ll, vector<int> > seen;
  VI diff(B);
  while (true) {
    VI asgn(B);
    num_trial += 1;
    REP(i, 0, B) {
      asgn[i] = rand() % 3;
    }
    // check
    ll hash = 0;
    REP(i, 0, B) {
      hash = (hash * a + asgn[i] + b) % b;
    }
    if (seen.count(hash) != 0) {
      // hit!
      VI old = seen[hash];
      bool non_zero = false;
      REP(i, 0, B) {
	diff[i] = asgn[i] - old[i];
	non_zero |= diff[i] != 0;
      }
      if (non_zero) {
	break;
      }
    }
    seen[hash] = asgn;
  }
  if (0) {
    cerr << "num_trial = " << num_trial << endl;
    REP(i, 0, B) {
      cerr << diff[i] << " ";
    }
    cerr << endl;
  }
  REP(i, 0, 10) {
    string former = calc(diff, i);
    REP(j, 0, 10) {
      string latter = calc(diff, j);
      cout << former + latter << endl;
    }
  }
}
