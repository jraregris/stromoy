module Stromoy
  # Normalize a 
  def self.normalize(seq)
    seq.bytes.collect do|c|
      c - 'a'.bytes.first
    end
  end
end

describe Stromoy do
  describe '#normalize' do
    it 'should return a seqeuence of relative numbers' do # TODO: better prose
      Stromoy.normalize('abc').should == [0,1,2]
      Stromoy.normalize('def').should == [3,4,5]
    end
  end
end
