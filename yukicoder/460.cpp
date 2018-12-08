#include <algorithm>
#include <bitset>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
/*
 * Find an assignment (result) s.t. xor_i a[i] * result[i] = b (in GF(2))
 * Returns true if such an assignment was found.
 * Verified by: yukicoder No.460 (https://yukicoder.me/submissions/302616)
 */
bool gauss_elim_gf2_i64(vector<ll> a, ll b, vector<bool> &result) {
  int n = a.size();
  int c = 0;
  vector<int> orig(n);
  REP(i, 0, n) orig[i] = i;
  vector<int> revmap;
  REP(r, 0, 64) {
    if (c >= n) {
      break;
    }
    int c2 = -1;
    REP(i, c, n) {
      if (a[i] & (1LL << r)) {
        c2 = i;
        break;
      }
    }
    if (c2 < 0) {
      revmap.push_back(-1);
      continue;
    }
    if (c != c2) {
      swap(a[c], a[c2]);
      swap(orig[c], orig[c2]);
    }
    ll rm = a[c] & -((1LL << r) << 1);
    a[c] ^= rm;
    REP(k, c + 1, n) {
      if (a[k] & (1LL << r)) {
        a[k] ^= rm;
      }
    }
    if (b & (1LL << r)) {
      b ^= rm;
    }
    revmap.push_back(c);
    c++;
  }
  // recover
  int rank = revmap.size();
  result.assign(n, false);
  for (int i = rank - 1; i >= 0; --i) {
    if (b & 1LL << i) {
      int c = revmap[i];
      if (c < 0) {
        return false;
      }
      b ^= a[c];
      result[orig[c]] = true;
    }
  }
  return b == 0;
}

int solve(ll board, int m, int n) {
  VL basis;
  VL varbasis;
  REP(i, 0, m) {
    REP(j, 0, n) {
      ll v = 0;
      REP(dx, -1, 2) {
	REP(dy, -1, 2) {
	  int ni = i + dx;
	  int nj = j + dy;
	  if (ni < 0 || ni >= m || nj < 0 || nj >= n) {
	    continue;
	  }
	  v |= 1LL << (ni * n + nj);
	}
      }
      if (i > 0 && j > 0) {
	basis.push_back(v);
      } else {
	varbasis.push_back(v);
      }
    }
  }
  int q = varbasis.size();
  int mi = 100;
  REP(bits, 0, 1 << q) {
    ll bd = board;
    REP(i, 0, q) {
      if (bits & 1 << i) {
	bd ^= varbasis[i];
      }
    }
    vector<bool> result;
    bool succ = gauss_elim_gf2_i64(basis, bd, result);
    if (succ) {
      // verify
      {
        ll sum = 0;
        REP(i, 0, result.size()) {
          if (result[i]) {
            sum ^= basis[i];
          }
        }
        assert(bd == sum);
      }
      int rank = count(result.begin(), result.end(), true);
      mi = min(mi, __builtin_popcount(bits) + rank);
    }
  }
  return mi >= 100 ? -1 : mi;
}

int main(void){
  int m, n;
  cin >> m >> n;
  ll board = 0;
  REP(i, 0, m) {
    REP(j, 0, n) {
      int tmp;
      cin >> tmp;
      board |= (ll)tmp << (i * n + j);
    }
  }
  int result = solve(board, m, n);
  if (result < 0) {
    cout << "Impossible" << endl;
  } else {
    cout << result << endl;
  }
}
