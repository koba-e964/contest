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

int pd[6] = {2,3,5,7,11,13};
int cd[6] = {4,6,8,9,10,12};

VL calc(int dis[6], int len, int pos, int rem) {
  VL ret(len, 0);
  if (pos >= 6) {
    ret[0] = rem > 0 ? 0 : 1;
    return ret;
  }
  REP(i, 0, rem + 1) {
    VL sub = calc(dis, len, pos + 1, rem - i);
    REP(j, 0, len) {
	if (j + dis[pos] * i < len) {
	  ret[j + dis[pos] * i] += sub[j];
	}
    }
  }
  return ret;
}

VL get_die(int p, int c) {
  int l = 13 * p + 12 * c + 1;
  VL ret(l, 0);
  VL sub1 = calc(pd, 13 * p + 1, 0, p);
  VL sub2 = calc(cd, 12 * c + 1, 0, c);
  REP(i, 0, 13 * p + 1) {
    REP(j, 0, 12 * c + 1) {
      ret[i + j] += sub1[i] * sub2[j];
    }
  }
  return ret;
}

vector<VL> mat_mul(vector<VL> &A, vector<VL> &B) {
  int n = A.size();
  int m = B.size();
  int l = B[0].size();
  vector<VL> result(n, VL(m));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	result[i][k] += A[i][j] * B[j][k];
	result[i][k] %= mod;
      }
    }
  }
  return result;
}

vector<VL> mat_pow(vector<VL> &mat, ll e) {
  int n = mat.size();
  vector<VL> sum(n, VL(n));
  REP(i, 0, n) { sum[i][i] = 1; }
  vector<VL> cur = mat;
  while (e > 0) {
    if (e % 2 == 1) {
      sum = mat_mul(sum, cur);
    }
    cur = mat_mul(cur, cur);
    e /= 2;
  }
  return sum;
}
  

int main(void){
  ll n;
  int p, c;
  cin >> n >> p >> c;
  VL die = get_die(p, c);
  if (0) {
    cerr << "die:";
    REP(i, 0, die.size()) {
      cerr << " " << die[i];
    }
    cerr << endl;
  }
  int len = 13 * p + 12 * c;
  vector<VL> mat(len, VL(len, 0));
  REP(i, 0, len) {
    mat[0][i] = die[i + 1];
  }
  REP(i, 0, len - 1) {
    mat[i + 1][i] = 1;
  }
  VL res(len);
  vector<VL> pw = mat_pow(mat, n - 1);
  REP(i, 0, min((ll)len, n)) {
    res[i] = pw[i][0];
    if (0) {
      cerr << "dp[" << n - 1 - i << "]=" << res[i] << endl;
    }
  }
  ll tot = 0;
  REP(i, 0, len) {
    REP(j, i + 1, len + 1) {
      tot += res[i] * die[j];
      tot %= mod;
    }
  }
  cout << tot << endl;
}
