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


int solve(int v) {
  int s = 0;
  while (v > 0) {
    int q = v % 10;
    s += q * q * q * q * q;
    v /= 10;
  }
  return s;
}

int main(void){
  int c = 0;
  REP(i, 2, 1000000) {
    c += i == solve(i) ? i : 0;
  }
  cout << c << endl;
}
