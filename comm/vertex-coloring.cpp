class Coloring {
private:
	int cmin;
	int n;
	vector<vector<int> > e;
public:
	/* n: number of edges, e: adjacent matrix of the graph */
	Coloring(int n, vector<vector<int> > e) : cmin(n+1), n(n), e(e) {
		if(n>=33) {
			assert(0); // n is too large
		}
	}
	int solve(void) {
		vector<int> col(n,0);
		col[0]=1;
		this->coloring(col, 1, 1);
		return cmin;
	}
private:
	void coloring(vector<int> bits, int pos, int cols) {
		if(cols >= cmin || cols > n) return;
		if(pos == n) {
			int res = cols;
			cmin = min(cmin, res);
			return;
		}
		for(int k = 0; k < cols; ++k) {
			bool f = true;
			for(int i = 0;i < n; ++i) {
				if((bits[k] & (1 << i)) && e[pos][i]) {
					f=false;
					break;
				}
			}
			if(f) {
				bits[k] ^= 1 << pos;
				coloring(bits, pos+1, cols);
				bits[k] ^= 1 << pos;
			}
		}
		bits[cols] = 1 << pos;
		coloring(bits, pos+1, cols+1);
		bits[cols] = 0;
	}
};
 