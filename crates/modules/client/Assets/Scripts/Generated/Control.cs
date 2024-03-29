// <auto-generated>
//     Generated by the protocol buffer compiler.  DO NOT EDIT!
//     source: control.proto
// </auto-generated>
#pragma warning disable 1591, 0612, 3021, 8981
#region Designer generated code

using pb = global::Google.Protobuf;
using pbc = global::Google.Protobuf.Collections;
using pbr = global::Google.Protobuf.Reflection;
using scg = global::System.Collections.Generic;
namespace Control {

  /// <summary>Holder for reflection information generated from control.proto</summary>
  public static partial class ControlReflection {

    #region Descriptor
    /// <summary>File descriptor for control.proto</summary>
    public static pbr::FileDescriptor Descriptor {
      get { return descriptor; }
    }
    private static pbr::FileDescriptor descriptor;

    static ControlReflection() {
      byte[] descriptorData = global::System.Convert.FromBase64String(
          string.Concat(
            "Cg1jb250cm9sLnByb3RvEgdjb250cm9sIkwKB0NvbnRyb2wSDgoGdGhydXN0",
            "GAEgASgBEhAKCGVsZXZhdG9yGAIgASgBEg8KB2FpbGVyb24YAyABKAESDgoG",
            "cnVkZGVyGAQgASgBYgZwcm90bzM="));
      descriptor = pbr::FileDescriptor.FromGeneratedCode(descriptorData,
          new pbr::FileDescriptor[] { },
          new pbr::GeneratedClrTypeInfo(null, null, new pbr::GeneratedClrTypeInfo[] {
            new pbr::GeneratedClrTypeInfo(typeof(global::Control.Control), global::Control.Control.Parser, new[]{ "Thrust", "Elevator", "Aileron", "Rudder" }, null, null, null, null)
          }));
    }
    #endregion

  }
  #region Messages
  [global::System.Diagnostics.DebuggerDisplayAttribute("{ToString(),nq}")]
  public sealed partial class Control : pb::IMessage<Control>
  #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      , pb::IBufferMessage
  #endif
  {
    private static readonly pb::MessageParser<Control> _parser = new pb::MessageParser<Control>(() => new Control());
    private pb::UnknownFieldSet _unknownFields;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pb::MessageParser<Control> Parser { get { return _parser; } }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public static pbr::MessageDescriptor Descriptor {
      get { return global::Control.ControlReflection.Descriptor.MessageTypes[0]; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    pbr::MessageDescriptor pb::IMessage.Descriptor {
      get { return Descriptor; }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Control() {
      OnConstruction();
    }

    partial void OnConstruction();

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Control(Control other) : this() {
      thrust_ = other.thrust_;
      elevator_ = other.elevator_;
      aileron_ = other.aileron_;
      rudder_ = other.rudder_;
      _unknownFields = pb::UnknownFieldSet.Clone(other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public Control Clone() {
      return new Control(this);
    }

    /// <summary>Field number for the "thrust" field.</summary>
    public const int ThrustFieldNumber = 1;
    private double thrust_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Thrust {
      get { return thrust_; }
      set {
        thrust_ = value;
      }
    }

    /// <summary>Field number for the "elevator" field.</summary>
    public const int ElevatorFieldNumber = 2;
    private double elevator_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Elevator {
      get { return elevator_; }
      set {
        elevator_ = value;
      }
    }

    /// <summary>Field number for the "aileron" field.</summary>
    public const int AileronFieldNumber = 3;
    private double aileron_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Aileron {
      get { return aileron_; }
      set {
        aileron_ = value;
      }
    }

    /// <summary>Field number for the "rudder" field.</summary>
    public const int RudderFieldNumber = 4;
    private double rudder_;
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public double Rudder {
      get { return rudder_; }
      set {
        rudder_ = value;
      }
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override bool Equals(object other) {
      return Equals(other as Control);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public bool Equals(Control other) {
      if (ReferenceEquals(other, null)) {
        return false;
      }
      if (ReferenceEquals(other, this)) {
        return true;
      }
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Thrust, other.Thrust)) return false;
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Elevator, other.Elevator)) return false;
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Aileron, other.Aileron)) return false;
      if (!pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.Equals(Rudder, other.Rudder)) return false;
      return Equals(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override int GetHashCode() {
      int hash = 1;
      if (Thrust != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Thrust);
      if (Elevator != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Elevator);
      if (Aileron != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Aileron);
      if (Rudder != 0D) hash ^= pbc::ProtobufEqualityComparers.BitwiseDoubleEqualityComparer.GetHashCode(Rudder);
      if (_unknownFields != null) {
        hash ^= _unknownFields.GetHashCode();
      }
      return hash;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public override string ToString() {
      return pb::JsonFormatter.ToDiagnosticString(this);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void WriteTo(pb::CodedOutputStream output) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      output.WriteRawMessage(this);
    #else
      if (Thrust != 0D) {
        output.WriteRawTag(9);
        output.WriteDouble(Thrust);
      }
      if (Elevator != 0D) {
        output.WriteRawTag(17);
        output.WriteDouble(Elevator);
      }
      if (Aileron != 0D) {
        output.WriteRawTag(25);
        output.WriteDouble(Aileron);
      }
      if (Rudder != 0D) {
        output.WriteRawTag(33);
        output.WriteDouble(Rudder);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(output);
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalWriteTo(ref pb::WriteContext output) {
      if (Thrust != 0D) {
        output.WriteRawTag(9);
        output.WriteDouble(Thrust);
      }
      if (Elevator != 0D) {
        output.WriteRawTag(17);
        output.WriteDouble(Elevator);
      }
      if (Aileron != 0D) {
        output.WriteRawTag(25);
        output.WriteDouble(Aileron);
      }
      if (Rudder != 0D) {
        output.WriteRawTag(33);
        output.WriteDouble(Rudder);
      }
      if (_unknownFields != null) {
        _unknownFields.WriteTo(ref output);
      }
    }
    #endif

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public int CalculateSize() {
      int size = 0;
      if (Thrust != 0D) {
        size += 1 + 8;
      }
      if (Elevator != 0D) {
        size += 1 + 8;
      }
      if (Aileron != 0D) {
        size += 1 + 8;
      }
      if (Rudder != 0D) {
        size += 1 + 8;
      }
      if (_unknownFields != null) {
        size += _unknownFields.CalculateSize();
      }
      return size;
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(Control other) {
      if (other == null) {
        return;
      }
      if (other.Thrust != 0D) {
        Thrust = other.Thrust;
      }
      if (other.Elevator != 0D) {
        Elevator = other.Elevator;
      }
      if (other.Aileron != 0D) {
        Aileron = other.Aileron;
      }
      if (other.Rudder != 0D) {
        Rudder = other.Rudder;
      }
      _unknownFields = pb::UnknownFieldSet.MergeFrom(_unknownFields, other._unknownFields);
    }

    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    public void MergeFrom(pb::CodedInputStream input) {
    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
      input.ReadRawMessage(this);
    #else
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, input);
            break;
          case 9: {
            Thrust = input.ReadDouble();
            break;
          }
          case 17: {
            Elevator = input.ReadDouble();
            break;
          }
          case 25: {
            Aileron = input.ReadDouble();
            break;
          }
          case 33: {
            Rudder = input.ReadDouble();
            break;
          }
        }
      }
    #endif
    }

    #if !GOOGLE_PROTOBUF_REFSTRUCT_COMPATIBILITY_MODE
    [global::System.Diagnostics.DebuggerNonUserCodeAttribute]
    [global::System.CodeDom.Compiler.GeneratedCode("protoc", null)]
    void pb::IBufferMessage.InternalMergeFrom(ref pb::ParseContext input) {
      uint tag;
      while ((tag = input.ReadTag()) != 0) {
        switch(tag) {
          default:
            _unknownFields = pb::UnknownFieldSet.MergeFieldFrom(_unknownFields, ref input);
            break;
          case 9: {
            Thrust = input.ReadDouble();
            break;
          }
          case 17: {
            Elevator = input.ReadDouble();
            break;
          }
          case 25: {
            Aileron = input.ReadDouble();
            break;
          }
          case 33: {
            Rudder = input.ReadDouble();
            break;
          }
        }
      }
    }
    #endif

  }

  #endregion

}

#endregion Designer generated code
