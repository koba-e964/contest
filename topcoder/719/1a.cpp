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
using namespace std;

class LongMansionDiv1 {
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
  typedef long long int ll;
public:
  long long minimalTime(vector <int> t, int sX, int sY, int eX, int eY) {
    ll cal = 0;
    if (sX > eX) { swap(sX, eX); }
    if (sY > eY) { swap(sY, eY); }
    REP(i, sX, eX + 1) {
      cal += t[i];
    }
    ll mi = 1e15;
    int n = t.size();
    REP(i, 0, n) {
      ll tmp = t[i] * ll(eY - sY);
      if (i < sX) {
	// sX < j <= eX counted once, i <= j <= sX counted twice
	tmp -= t[i];
	REP(j, i, sX + 1) {
	  tmp += 2 * t[j];
	}
	REP(j, sX + 1, eX + 1) {
	  tmp += t[j];
	}
      } else if (i > eX) {
	tmp -= t[i];
	REP(j, eX, i + 1) {
	  tmp += 2 * t[j];
	}
	REP(j, sX, eX) {
	  tmp += t[j];
	}
      } else {
	tmp += cal;
      }
      mi = min(mi, tmp);
    }
    return mi;
  }
};
