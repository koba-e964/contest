#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

int n, s, t;

int main(void){
  cin >> n >> s >> t;
  int x = 0;
  int cnt = 0;
  REP(i, 0, n) {
    int q;
    cin >> q;
    x += q;
    if (x >= s && x <= t) {
      cnt ++;
    }
  }
  cout << cnt << endl;
}
